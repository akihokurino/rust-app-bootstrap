use crate::errors::AppError;
use crate::rdb::{session_manager, types};
#[allow(unused)]
use once_cell;
use tokio::sync::{Mutex, OnceCell};

mod ddb;
pub mod domain;
pub mod errors;
mod lambda;
pub mod rdb;
mod s3;
mod ses;
mod sns;
mod ssm;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Clone)]
pub struct Resolver {
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

    let session_manager = session_manager::SessionManager::from_env().await?;
    let user_repository = types::user::UserRepository {};
    let order_repository = types::order::OrderRepository {};
    let order_detail_repository = types::order_detail::OrderDetailRepository {};

    let resolver = Resolver {
        session_manager,
        user_repository,
        order_repository,
        order_detail_repository,
    };

    RESOLVER.set(resolver).unwrap();
    Ok(RESOLVER.get().unwrap())
}
