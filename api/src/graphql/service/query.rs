use crate::graphql::data_loader::{OrderDataLoader, UserDataLoader};
use crate::graphql::service::types::order::{Order, OrderPayload};
use crate::graphql::service::types::user::{Me, MePayload, User, UserListPayload, UserPayload};
use crate::graphql::service::AppContext;
use crate::graphql::GraphResult;
use app::domain::types::image_size::ImageSize;
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

    async fn pre_sign_download(
        &self,
        ctx: &Context<'_>,
        key: String,
        size: Option<ImageSize>,
    ) -> GraphResult<String> {
        let _uid = ctx.verified_user_id()?;
        let app = ctx.data::<app::App>()?;
        let asset_key = key.try_into().map_err(BadRequest.withf())?;

        let presign_url = match (size, &app.image_cdn) {
            (Some(size), Some(cdn)) => cdn.presign_for_get(&asset_key, size.into()).await?,
            _ => app.storage.presign_for_get(&asset_key).await?,
        };

        Ok(presign_url.to_string())
    }

    async fn me(&self, ctx: &Context<'_>) -> GraphResult<MePayload> {
        let me = ctx.verified_user().await?;
        Ok(Me::from(me).into())
    }

    async fn users(&self, ctx: &Context<'_>) -> GraphResult<UserListPayload> {
        let app = ctx.data::<app::App>()?;
        let conn = app.db_session.conn();
        let users = app.user_repository.find(conn).await?;
        Ok(users.into_iter().map(User::from).collect::<Vec<_>>().into())
    }

    async fn user(&self, ctx: &Context<'_>, id: ID) -> GraphResult<UserPayload> {
        let user_loader = ctx.data::<UserDataLoader>()?;
        let user = user_loader.load_one(id.0.into()).await?;
        let user = user.ok_or_else(|| BadRequest.with("user not found"))?;
        Ok(User::from(user).into())
    }

    async fn order(&self, ctx: &Context<'_>, id: ID) -> GraphResult<OrderPayload> {
        let order_loader = ctx.data::<OrderDataLoader>()?;
        let order = order_loader.load_one(id.0.into()).await?;
        let order = order.ok_or_else(|| BadRequest.with("order not found"))?;
        Ok(Order::from(order).into())
    }
}
