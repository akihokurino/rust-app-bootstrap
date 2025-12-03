use crate::graphql::data_loader::order::OrderDataLoader;
use crate::graphql::data_loader::user::UserDataLoader;
use crate::graphql::service::types::user::User;
use crate::graphql::shared::types::DateTime;
use crate::graphql::GraphResult;
use app::domain;
use app::errors::Kind::*;
use async_graphql::{Context, Object, ID};
use derive_more::From;

#[derive(Debug, Clone, From)]
pub struct Order(domain::order::Order);
#[Object]
impl Order {
    async fn id(&self) -> ID {
        ID::from(self.0.id.as_str())
    }

    async fn user(&self, ctx: &Context<'_>) -> GraphResult<User> {
        let user_loader = ctx.data::<UserDataLoader>()?;
        let user_id = self.0.user_id.clone();
        let user = user_loader.load_one(user_id).await?;
        let user = user.ok_or_else(|| NotFound.with("user not found"))?;
        Ok(User::from(user))
    }

    async fn details(&self, ctx: &Context<'_>) -> GraphResult<Vec<OrderDetail>> {
        let app = ctx.data::<app::App>()?;
        let conn = app.db_session.conn();
        let details = app
            .order_detail_repository
            .find_by_order(conn, &self.0.id)
            .await?;
        Ok(details.into_iter().map(|v| v.into()).collect())
    }

    async fn created_at(&self) -> DateTime {
        self.0.created_at.into()
    }

    async fn updated_at(&self) -> DateTime {
        self.0.updated_at.into()
    }
}

#[derive(Debug, Clone, From)]
pub struct OrderDetail(domain::order::detail::Detail);
#[Object]
impl OrderDetail {
    async fn id(&self) -> ID {
        ID::from(self.0.id.as_str())
    }

    async fn order(&self, ctx: &Context<'_>) -> GraphResult<Order> {
        let order_loader = ctx.data::<OrderDataLoader>()?;
        let order = order_loader.load_one(self.0.order_id.clone()).await?;
        let order = order.ok_or_else(|| BadRequest.with("order not found"))?;
        Ok(order.into())
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
