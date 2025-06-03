use async_graphql::dataloader::*;
use async_graphql::*;
use core::domain;
use core::domain::IntoIdMap;
use core::errors::AppError;
use std::collections::HashMap;

pub struct UserLoader {
    core_resolver: core::Resolver,
}
impl Loader<domain::user::Id> for UserLoader {
    type Value = domain::user::User;
    type Error = AppError;

    async fn load(
        &self,
        keys: &[domain::user::Id],
    ) -> Result<HashMap<domain::user::Id, Self::Value>, Self::Error> {
        let pool = self.core_resolver.session_manager.pool();
        let ids = keys.into_iter().collect::<Vec<_>>();
        let items = self
            .core_resolver
            .user_repository
            .get_multi(pool, ids)
            .await?;
        Ok(items.into_id_map())
    }
}

pub type UserDataLoader = DataLoader<UserLoader>;

pub fn new_loader(core_resolver: core::Resolver) -> UserDataLoader {
    DataLoader::new(UserLoader { core_resolver }, tokio::spawn)
}
