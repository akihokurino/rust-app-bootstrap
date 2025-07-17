use async_graphql::dataloader::*;
use async_graphql::*;
use app::domain;
use app::domain::IntoIdMap;
use app::errors::AppError;
use std::collections::HashMap;

pub struct UserLoader {
    resolver: app::Resolver,
}
impl Loader<domain::user::Id> for UserLoader {
    type Value = domain::user::User;
    type Error = AppError;

    async fn load(
        &self,
        keys: &[domain::user::Id],
    ) -> Result<HashMap<domain::user::Id, Self::Value>, Self::Error> {
        let pool = self.resolver.session_manager.pool();
        let ids = keys.into_iter().collect::<Vec<_>>();
        let items = self
            .resolver
            .user_repository
            .get_multi(pool, ids)
            .await?;
        Ok(items.into_id_map())
    }
}

pub type UserDataLoader = DataLoader<UserLoader>;

pub fn new_loader(resolver: app::Resolver) -> UserDataLoader {
    DataLoader::new(UserLoader { resolver }, tokio::spawn)
}
