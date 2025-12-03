use app::domain;
use app::domain::IntoIdMap;
use app::errors::AppError;
use async_graphql::dataloader::*;
use async_graphql::*;
use std::collections::HashMap;

pub struct UserLoader {
    app: app::App,
}
impl Loader<domain::user::Id> for UserLoader {
    type Value = domain::user::User;
    type Error = AppError;

    async fn load(
        &self,
        keys: &[domain::user::Id],
    ) -> Result<HashMap<domain::user::Id, Self::Value>, Self::Error> {
        let db = self.app.session_manager.db();
        let ids = keys.into_iter().collect::<Vec<_>>();
        let items = self.app.user_repository.get_multi(db, ids).await?;
        Ok(items.into_id_map())
    }
}

pub type UserDataLoader = DataLoader<UserLoader>;

pub fn new_loader(app: app::App) -> UserDataLoader {
    DataLoader::new(UserLoader { app }, tokio::spawn)
}
