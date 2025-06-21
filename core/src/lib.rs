use crate::errors::AppError;
use crate::infra::{s3, sns};
use aws_config::BehaviorVersion;
use infra::rdb::{session_manager, types};
use infra::ssm;
#[allow(unused)]
use once_cell;
use tokio::sync::{Mutex, OnceCell};

pub mod domain;
mod env;
pub mod errors;
pub mod infra;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Clone)]
pub struct Resolver {
    pub envs: env::Environments,
    pub s3: s3::Adapter,
    pub sns: sns::Adapter,
    pub session_manager: session_manager::SessionManager,
    pub user_repository: types::user::UserRepository,
    pub order_repository: types::order::OrderRepository,
    pub order_detail_repository: types::order_detail::OrderDetailRepository,
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

    println!("Loaded environment variables: {:?}", envs);

    let s3 = s3::Adapter::new(
        aws_sdk_s3::Client::new(&aws_config),
        envs.s3_bucket_name.clone(),
    );
    let sns = sns::Adapter::new(aws_sdk_sns::Client::new(&aws_config));
    let session_manager = session_manager::SessionManager::new(&envs.database_url).await?;
    let user_repository = types::user::UserRepository {};
    let order_repository = types::order::OrderRepository {};
    let order_detail_repository = types::order_detail::OrderDetailRepository {};

    let resolver = Resolver {
        envs,
        s3,
        sns,
        session_manager,
        user_repository,
        order_repository,
        order_detail_repository,
    };

    RESOLVER.set(resolver).unwrap();
    Ok(RESOLVER.get().unwrap())
}
