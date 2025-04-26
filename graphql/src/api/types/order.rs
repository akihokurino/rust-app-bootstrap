use crate::api::types::user::User;
use crate::shared::types::DateTime;
use crate::GraphResult;
use async_graphql::{Object, ID};
use derive_more::From;

#[derive(Debug, Clone, From)]
pub struct Order(domain::types::order::Order);
#[Object]
impl Order {
    async fn id(&self) -> ID {
        ID::from(self.0.id.as_str())
    }

    async fn user(&self) -> GraphResult<User> {
        // TODO: implement
        let user = domain::types::user::User {
            id: domain::types::user::Id::generate(),
            name: domain::types::user::Name::try_from("sample".to_string()).unwrap(),
            created_at: domain::types::time::now(),
            updated_at: domain::types::time::now(),
        };
        Ok(User::from(user))
    }

    async fn details(&self) -> GraphResult<Vec<OrderDetail>> {
        // TODO: implement
        let detail = domain::types::order::detail::Detail {
            id: domain::types::order::detail::Id::generate(),
            order_id: domain::types::order::Id::generate(),
            quantity: 1,
            created_at: domain::types::time::now(),
            updated_at: domain::types::time::now(),
        };
        Ok(vec![OrderDetail::from(detail)])
    }

    async fn created_at(&self) -> DateTime {
        self.0.created_at.into()
    }

    async fn updated_at(&self) -> DateTime {
        self.0.updated_at.into()
    }
}

#[derive(Debug, Clone, From)]
pub struct OrderDetail(domain::types::order::detail::Detail);
#[Object]
impl OrderDetail {
    async fn id(&self) -> ID {
        ID::from(self.0.id.as_str())
    }

    async fn order(&self) -> GraphResult<Order> {
        // TODO: implement
        let order = domain::types::order::Order {
            id: domain::types::order::Id::generate(),
            user_id: domain::types::user::Id::generate(),
            created_at: domain::types::time::now(),
            updated_at: domain::types::time::now(),
        };
        Ok(Order::from(order))
    }

    async fn quantity(&self) -> u32 {
        self.0.quantity
    }

    async fn created_at(&self) -> DateTime {
        self.0.created_at.into()
    }

    async fn updated_at(&self) -> DateTime {
        self.0.updated_at.into()
    }
}
