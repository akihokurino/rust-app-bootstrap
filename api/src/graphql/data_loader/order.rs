use async_graphql::dataloader::*;
use async_graphql::*;
use core::domain;
use core::domain::IntoIdMap;
use core::errors::AppError;
use std::collections::HashMap;

pub struct OrderLoader {
    core_resolver: core::Resolver,
}
impl Loader<domain::order::Id> for OrderLoader {
    type Value = domain::order::Order;
    type Error = AppError;

    async fn load(
        &self,
        keys: &[domain::order::Id],
    ) -> Result<HashMap<domain::order::Id, Self::Value>, Self::Error> {
        let pool = self.core_resolver.session_manager.pool();
        let ids = keys.into_iter().collect::<Vec<_>>();
        let items = self
            .core_resolver
            .order_repository
            .get_multi(pool, ids)
            .await?;
        Ok(items.into_id_map())
    }
}

pub type OrderDataLoader = DataLoader<OrderLoader>;

pub fn new_loader(core_resolver: core::Resolver) -> OrderDataLoader {
    DataLoader::new(OrderLoader { core_resolver }, tokio::spawn)
}
