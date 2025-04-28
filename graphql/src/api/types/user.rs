use crate::api::types::order::Order;
use crate::shared::types::DateTime;
use crate::GraphResult;
use async_graphql::{Object, ID};
use derive_more::From;

#[derive(Debug, Clone, From)]
pub struct Me(domain::models::user::User);
#[Object]
impl Me {
    async fn id(&self) -> ID {
        ID::from(self.0.id.as_str())
    }

    async fn name(&self) -> String {
        self.0.name.to_string()
    }

    async fn orders(&self) -> GraphResult<Vec<Order>> {
        // TODO: implement
        let order = domain::models::order::Order {
            id: domain::models::order::Id::generate(),
            user_id: domain::models::user::Id::generate(),
            created_at: domain::models::time::now(),
            updated_at: domain::models::time::now(),
        };
        Ok(vec![Order::from(order)])
    }

    async fn created_at(&self) -> DateTime {
        self.0.created_at.into()
    }

    async fn updated_at(&self) -> DateTime {
        self.0.updated_at.into()
    }
}

#[derive(Debug, Clone, From)]
pub struct User(domain::models::user::User);
#[Object]
impl User {
    async fn id(&self) -> ID {
        ID::from(self.0.id.as_str())
    }

    async fn name(&self) -> String {
        self.0.name.to_string()
    }

    async fn created_at(&self) -> DateTime {
        self.0.created_at.into()
    }

    async fn updated_at(&self) -> DateTime {
        self.0.updated_at.into()
    }
}
