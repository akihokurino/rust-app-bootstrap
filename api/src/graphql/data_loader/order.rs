use app::domain;
use app::domain::IntoIdMap;
use app::errors::AppError;
use async_graphql::dataloader::*;
use async_graphql::*;
use std::collections::HashMap;

pub struct OrderLoader {
    app: app::App,
}
impl Loader<domain::order::Id> for OrderLoader {
    type Value = domain::order::Order;
    type Error = AppError;

    async fn load(
        &self,
        keys: &[domain::order::Id],
    ) -> Result<HashMap<domain::order::Id, Self::Value>, Self::Error> {
        let db = self.app.session_manager.db();
        let ids = keys.into_iter().collect::<Vec<_>>();
        let items = self.app.order_repository.get_multi(db, ids).await?;
        Ok(items.into_id_map())
    }
}

pub type OrderDataLoader = DataLoader<OrderLoader>;

pub fn new_loader(app: app::App) -> OrderDataLoader {
    DataLoader::new(OrderLoader { app }, tokio::spawn)
}
