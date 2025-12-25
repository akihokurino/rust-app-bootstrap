use crate::adapter::{
    AdminAuth, DBSession, ImageCdn, RemoteFunction, Storage, TaskQueue, UserAuth,
};
use crate::domain::order::detail::OrderDetailRepository;
use crate::domain::order::OrderRepository;
use crate::domain::user::UserRepository;
use crate::errors::AppError;
use crate::errors::Kind::Internal;
use crate::infra::{cloudfront, cognito, firebase};
use aws_config::BehaviorVersion;
use google_fcm1::{hyper, hyper_rustls};
use google_identitytoolkit3::oauth2::ServiceAccountAuthenticator;
use google_identitytoolkit3::IdentityToolkit;
use infra::rdb::{repository, session_manager};
use infra::ssm;
use infra::{lambda, s3, sns, sqs};
#[allow(unused)]
use once_cell;
use std::sync::Arc;
use tokio::sync::{Mutex, OnceCell};

pub mod adapter;
pub mod domain;
mod env;
pub mod errors;
mod infra;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Clone)]
pub struct App {
    pub env: env::Env,
    pub storage: Arc<dyn Storage>,
    pub image_cdn: Option<Arc<dyn ImageCdn>>,
    pub user_auth: Arc<dyn UserAuth>,
    pub admin_auth: Arc<dyn AdminAuth>,
    pub sns_task_queue: Arc<dyn TaskQueue>,
    pub sqs_task_queue: Arc<dyn TaskQueue>,
    pub remote_function: Arc<dyn RemoteFunction>,
    pub db_session: Arc<dyn DBSession>,
    pub user_repository: Arc<dyn UserRepository>,
    pub order_repository: Arc<dyn OrderRepository>,
    pub order_detail_repository: Arc<dyn OrderDetailRepository>,
}

impl std::fmt::Debug for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("App")
            .field("env", &self.env)
            .finish_non_exhaustive()
    }
}

static APP: OnceCell<App> = OnceCell::const_new();
static INIT_LOCK: Mutex<()> = Mutex::const_new(());

pub async fn app() -> AppResult<&'static App> {
    if let Some(r) = APP.get() {
        return Ok(r);
    }

    let _guard = INIT_LOCK.lock().await;
    if let Some(r) = APP.get() {
        return Ok(r);
    }

    let aws_config = aws_config::defaults(BehaviorVersion::latest()).load().await;
    let ssm_parameter_name =
        std::env::var("SSM_DOTENV_PARAMETER_NAME").expect("SSM_DOTENV_PARAMETER_NAME should set");
    let ssm = ssm::Adapter::new(aws_sdk_ssm::Client::new(&aws_config), ssm_parameter_name);
    ssm.load_dotenv().await?;
    let envs = env::Env::new();

    let storage: Arc<dyn Storage> = Arc::new(s3::Adapter::new(
        aws_sdk_s3::Client::new(&aws_config),
        envs.s3_bucket_name.clone(),
    ));
    let image_cdn: Option<Arc<dyn ImageCdn>> =
        if let (Some(domain), Some(key_pair_id), Some(private_key)) = (
            envs.cloudfront_domain.clone(),
            envs.cloudfront_key_pair_id.clone(),
            envs.cloudfront_private_key.clone(),
        ) {
            Some(Arc::new(cloudfront::Adapter::new(
                domain,
                key_pair_id,
                private_key,
            )?))
        } else {
            None
        };
    let admin_auth: Arc<dyn AdminAuth> = Arc::new(cognito::Adapter::new(
        aws_sdk_cognitoidentityprovider::Client::new(&aws_config),
        envs.cognito_admin_user_pool_id.clone(),
    ));

    let sns_task_queue: Arc<dyn TaskQueue> =
        Arc::new(sns::Adapter::new(aws_sdk_sns::Client::new(&aws_config)));
    let sqs_task_queue: Arc<dyn TaskQueue> =
        Arc::new(sqs::Adapter::new(aws_sdk_sqs::Client::new(&aws_config)));
    let remote_function: Arc<dyn RemoteFunction> = Arc::new(lambda::Adapter::new(
        aws_sdk_lambda::Client::new(&aws_config),
    ));

    let db_session: Arc<dyn DBSession> =
        Arc::new(session_manager::SessionManager::new(&envs.database_url).await?);
    let user_repository: Arc<dyn UserRepository> = Arc::new(repository::user::Repository::new());
    let order_repository: Arc<dyn OrderRepository> = Arc::new(repository::order::Repository::new());
    let order_detail_repository: Arc<dyn OrderDetailRepository> =
        Arc::new(repository::order_detail::Repository::new());

    let user_auth: Arc<dyn UserAuth> = if let Some(ref cred) = envs.google_application_credentials {
        let credentials_json = serde_json::to_string(cred).map_err(Internal.from_srcf())?;
        std::fs::write("/tmp/gcp-key.json", credentials_json).map_err(Internal.from_srcf())?;
        unsafe {
            std::env::set_var("GOOGLE_APPLICATION_CREDENTIALS", "/tmp/gcp-key.json");
        }

        let google_account = ServiceAccountAuthenticator::with_client(
            cred.clone(),
            hyper::Client::builder().build(
                hyper_rustls::HttpsConnectorBuilder::new()
                    .with_native_roots()
                    .unwrap()
                    .https_or_http()
                    .enable_http1()
                    .enable_http2()
                    .build(),
            ),
        )
        .build()
        .await
        .unwrap();

        let google_http_conn = hyper::Client::builder().build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .unwrap()
                .https_or_http()
                .enable_http1()
                .enable_http2()
                .build(),
        );

        let firebase_auth = firebase::auth::Adapter::new(
            envs.google_project_id.clone(),
            IdentityToolkit::new(google_http_conn.clone(), google_account.clone()),
        )
        .await;

        Arc::new(firebase_auth)
    } else {
        Arc::new(firebase::mock::MockAdapter::new())
    };

    let app = App {
        env: envs,
        storage,
        image_cdn,
        user_auth,
        admin_auth,
        sns_task_queue,
        sqs_task_queue,
        remote_function,
        db_session,
        user_repository,
        order_repository,
        order_detail_repository,
    };

    APP.set(app).unwrap();
    Ok(APP.get().unwrap())
}

pub fn init_log() {
    infra::log::init();
}
