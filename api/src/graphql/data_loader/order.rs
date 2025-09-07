use app::domain;
use app::domain::IntoIdMap;
use app::errors::AppError;
use async_graphql::dataloader::*;
use async_graphql::*;
use std::collections::HashMap;

pub struct OrderLoader {
    resolver: app::Resolver,
}
impl Loader<domain::order::Id> for OrderLoader {
    type Value = domain::order::Order;
    type Error = AppError;

    async fn load(
        &self,
        keys: &[domain::order::Id],
    ) -> Result<HashMap<domain::order::Id, Self::Value>, Self::Error> {
        let db = self.resolver.session_manager.db();
        let ids = keys.into_iter().collect::<Vec<_>>();
        let items = self.resolver.order_repository.get_multi(db, ids).await?;
        Ok(items.into_id_map())
    }
}

pub type OrderDataLoader = DataLoader<OrderLoader>;

pub fn new_loader(resolver: app::Resolver) -> OrderDataLoader {
    DataLoader::new(OrderLoader { resolver }, tokio::spawn)
}
