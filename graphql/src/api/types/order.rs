use crate::api::types::user::User;
use crate::shared::types::DateTime;
use crate::GraphResult;
use async_graphql::{Object, ID};
use derive_more::From;

#[derive(Debug, Clone, From)]
pub struct Order(domain::models::order::Order);
#[Object]
impl Order {
    async fn id(&self) -> ID {
        ID::from(self.0.id.as_str())
    }

    async fn user(&self) -> GraphResult<User> {
        // TODO: implement
        let user = domain::models::user::User {
            id: domain::models::user::Id::generate(),
            name: domain::models::user::Name::try_from("sample".to_string()).unwrap(),
            created_at: domain::models::time::now(),
            updated_at: domain::models::time::now(),
        };
        Ok(User::from(user))
    }

    async fn details(&self) -> GraphResult<Vec<OrderDetail>> {
        // TODO: implement
        let detail = domain::models::order::detail::Detail {
            id: domain::models::order::detail::Id::generate(),
            order_id: domain::models::order::Id::generate(),
            product_name: domain::models::order::detail::Name::try_from("sample".to_string())
                .unwrap(),
            quantity: 1,
            created_at: domain::models::time::now(),
            updated_at: domain::models::time::now(),
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
pub struct OrderDetail(domain::models::order::detail::Detail);
#[Object]
impl OrderDetail {
    async fn id(&self) -> ID {
        ID::from(self.0.id.as_str())
    }

    async fn order(&self) -> GraphResult<Order> {
        // TODO: implement
        let order = domain::models::order::Order {
            id: domain::models::order::Id::generate(),
            user_id: domain::models::user::Id::generate(),
            created_at: domain::models::time::now(),
            updated_at: domain::models::time::now(),
        };
        Ok(Order::from(order))
    }

    async fn product_name(&self) -> String {
        self.0.product_name.to_string()
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
