use domain::AppResult;
use once_cell::sync::OnceCell;

mod schema;
mod session_manager;
mod types;

pub struct Resolver {
    pub session_manager: session_manager::SessionManager,
    pub user_repository: types::user::UserRepository,
}

static RESOLVER: OnceCell<Resolver> = OnceCell::new();

pub fn resolver() -> AppResult<&'static Resolver> {
    RESOLVER.get_or_try_init(|| {
        let session_manager = session_manager::SessionManager::from_env()?;
        let user_repository = types::user::UserRepository {};
        Ok(Resolver {
            session_manager,
            user_repository,
        })
    })
}
