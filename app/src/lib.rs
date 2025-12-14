use crate::adapter::{AdminAuth, DBSession, RemoteFunction, Storage, TaskQueue};
use crate::domain::order::detail::OrderDetailRepository;
use crate::domain::order::OrderRepository;
use crate::domain::user::UserRepository;
use crate::errors::AppError;
use crate::infra::cognito;
use aws_config::BehaviorVersion;
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

    let app = App {
        env: envs,
        storage,
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
