use domain::AppResult;
use once_cell::sync::OnceCell;

mod macros;
mod schema;
mod session_manager;
mod types;

#[derive(Debug, Clone)]
pub struct Resolver {
    pub session_manager: session_manager::SessionManager,
    pub user_repository: types::user::UserRepository,
    pub order_repository: types::order::OrderRepository,
}

static RESOLVER: OnceCell<Resolver> = OnceCell::new();

pub async fn resolver() -> AppResult<&'static Resolver> {
    RESOLVER.get_or_try_init(|| {
        let session_manager = session_manager::SessionManager::from_env()?;
        let user_repository = types::user::UserRepository {};
        let order_repository = types::order::OrderRepository {};
        Ok(Resolver {
            session_manager,
            user_repository,
            order_repository,
        })
    })
}
