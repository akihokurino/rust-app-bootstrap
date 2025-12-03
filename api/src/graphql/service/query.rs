use crate::graphql::data_loader::order::OrderDataLoader;
use crate::graphql::data_loader::user::UserDataLoader;
use crate::graphql::service::types::order::Order;
use crate::graphql::service::types::user::{Me, User};
use crate::graphql::service::AppContext;
use crate::graphql::GraphResult;
use app::errors::Kind::BadRequest;
use async_graphql::{Context, MergedObject, Object, ID};

#[derive(MergedObject, Default)]
pub struct QueryRoot(DefaultQuery);

#[derive(Default)]
pub struct DefaultQuery;
#[Object]
impl DefaultQuery {
    async fn health(&self) -> String {
        "ok".to_string()
    }

    async fn pre_sign_download(&self, ctx: &Context<'_>, key: String) -> GraphResult<String> {
        let _uid = ctx.verified_user_id()?;
        let app = ctx.data::<app::App>()?;
        let presign_url = app
            .s3
            .pre_sign_for_get(&key.try_into().map_err(BadRequest.withf())?)
            .await?;
        Ok(presign_url.to_string())
    }

    async fn me(&self, ctx: &Context<'_>) -> GraphResult<Me> {
        let uid = ctx.verified_user_id()?;
        let user_loader = ctx.data::<UserDataLoader>()?;
        let user = user_loader.load_one(uid).await?;
        let user = user.ok_or_else(|| BadRequest.with("user not found"))?;
        Ok(user.into())
    }

    async fn users(&self, ctx: &Context<'_>) -> GraphResult<Vec<User>> {
        let app = ctx.data::<app::App>()?;
        let db = app.session_manager.db();
        let users = app.user_repository.find(db).await?;
        Ok(users.into_iter().map(|v| v.into()).collect())
    }

    async fn user(&self, ctx: &Context<'_>, id: ID) -> GraphResult<User> {
        let user_loader = ctx.data::<UserDataLoader>()?;
        let user = user_loader.load_one(id.0.into()).await?;
        let user = user.ok_or_else(|| BadRequest.with("user not found"))?;
        Ok(user.into())
    }

    async fn order(&self, ctx: &Context<'_>, id: ID) -> GraphResult<Order> {
        let order_loader = ctx.data::<OrderDataLoader>()?;
        let order = order_loader.load_one(id.0.into()).await?;
        let order = order.ok_or_else(|| BadRequest.with("order not found"))?;
        Ok(order.into())
    }
}
