use crate::graphql::service::types::order::Order;
use crate::graphql::shared::types::enum_value::Gender;
use crate::graphql::shared::types::{Date, DateTime};
use crate::graphql::GraphResult;
use app::domain;
use async_graphql::{Context, Object, ID};
use derive_more::From;

#[derive(Debug, Clone, From)]
pub struct Me(domain::user::User);
#[Object]
impl Me {
    async fn id(&self) -> ID {
        ID::from(self.0.id.as_str())
    }

    async fn name(&self) -> String {
        self.0.name.to_string()
    }

    async fn birthdate(&self) -> Date {
        self.0.birthdate.into()
    }

    async fn gender(&self) -> Gender {
        self.0.gender.into()
    }

    async fn orders(&self, ctx: &Context<'_>) -> GraphResult<Vec<Order>> {
        let app = ctx.data::<app::App>()?;
        let conn = app.db_session.conn();
        let orders = app.order_repository.find_by_user(conn, &self.0.id).await?;
        Ok(orders.into_iter().map(|v| v.into()).collect())
    }

    async fn created_at(&self) -> DateTime {
        self.0.created_at.into()
    }

    async fn updated_at(&self) -> DateTime {
        self.0.updated_at.into()
    }
}

#[derive(Debug, Clone, From)]
pub struct User(domain::user::User);
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
