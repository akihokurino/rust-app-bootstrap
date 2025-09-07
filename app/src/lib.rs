use crate::errors::AppError;
use crate::infra::{lambda, s3, sns};
use aws_config::BehaviorVersion;
use infra::rdb::{repository, session_manager};
use infra::ssm;
#[allow(unused)]
use once_cell;
use tokio::sync::{Mutex, OnceCell};

pub mod domain;
mod env;
pub mod errors;
pub mod infra;
pub mod model;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Clone)]
pub struct Resolver {
    pub envs: env::Environments,
    pub s3: s3::Adapter,
    pub sns: sns::Adapter,
    pub lambda: lambda::Adapter,
    pub session_manager: session_manager::SessionManager,
    pub user_repository: repository::user::Repository,
    pub order_repository: repository::order::Repository,
    pub order_detail_repository: repository::order_detail::Repository,
}

static RESOLVER: OnceCell<Resolver> = OnceCell::const_new();
static INIT_LOCK: Mutex<()> = Mutex::const_new(());

pub async fn resolver() -> AppResult<&'static Resolver> {
    if let Some(r) = RESOLVER.get() {
        return Ok(r);
    }

    let _guard = INIT_LOCK.lock().await;
    if let Some(r) = RESOLVER.get() {
        return Ok(r);
    }

    let aws_config = aws_config::defaults(BehaviorVersion::latest()).load().await;

    let ssm_parameter_name =
        std::env::var("SSM_DOTENV_PARAMETER_NAME").expect("SSM_DOTENV_PARAMETER_NAME should set");
    let ssm = ssm::Adapter::new(aws_sdk_ssm::Client::new(&aws_config), ssm_parameter_name);
    ssm.load_dotenv().await?;
    let envs = env::Environments::new();

    let s3 = s3::Adapter::new(
        aws_sdk_s3::Client::new(&aws_config),
        envs.s3_bucket_name.clone(),
    );
    let sns = sns::Adapter::new(aws_sdk_sns::Client::new(&aws_config));
    let lambda = lambda::Adapter::new(aws_sdk_lambda::Client::new(&aws_config));

    let session_manager = session_manager::SessionManager::new(&envs.database_url).await?;
    let user_repository = repository::user::Repository {};
    let order_repository = repository::order::Repository {};
    let order_detail_repository = repository::order_detail::Repository {};

    let resolver = Resolver {
        envs,
        s3,
        sns,
        lambda,
        session_manager,
        user_repository,
        order_repository,
        order_detail_repository,
    };

    RESOLVER.set(resolver).unwrap();
    Ok(RESOLVER.get().unwrap())
}
