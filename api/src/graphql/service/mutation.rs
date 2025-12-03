use crate::graphql::service::types::order::Order;
use crate::graphql::service::types::user::Me;
use crate::graphql::service::AppContext;
use crate::graphql::service::AppResult;
use crate::graphql::shared::types::BoolPayload;
use crate::graphql::GraphResult;
use app::domain;
use app::errors::Kind::BadRequest;
use app::errors::Kind::Internal;
use async_graphql::{Context, Enum, InputObject, MergedObject, Object, SimpleObject};
use rand::Rng;

#[derive(MergedObject, Default)]
pub struct MutationRoot(DefaultMutation);

#[derive(Default)]
pub struct DefaultMutation;
#[Object]
impl DefaultMutation {
    async fn pre_sign_upload(
        &self,
        ctx: &Context<'_>,
        input: PreSignUploadInput,
    ) -> GraphResult<PreSignUploadPayload> {
        let uid = ctx.verified_user_id()?;
        let app = ctx.data::<app::App>()?;

        let file_id = base_62::encode(&rand::rng().random::<[u8; 16]>());

        let key = format!("{}/{}/{}", input.path.path_string(), uid.as_str(), file_id);
        let url = app
            .s3
            .pre_sign_for_upload(&key.clone().try_into().map_err(Internal.withf())?)
            .await?;
        Ok(PreSignUploadPayload {
            file_id,
            key,
            url: url.to_string(),
        })
    }

    async fn call_async_task(&self, ctx: &Context<'_>) -> GraphResult<BoolPayload> {
        let app = ctx.data::<app::App>()?;
        let payload = app::infra::sns::types::AsyncTaskPayload {
            name: "My Async Task".to_string(),
        };
        app.sns
            .publish(payload, app.env.sns_async_task_topic_arn.clone())
            .await?;
        Ok(true.into())
    }

    async fn call_sync_task(&self, ctx: &Context<'_>) -> GraphResult<BoolPayload> {
        let app = ctx.data::<app::App>()?;
        let payload = app::infra::lambda::types::SyncTaskPayload {
            name: "My Sync Task".to_string(),
        };
        let resp: app::infra::lambda::types::SyncTaskResponse = app
            .lambda
            .invoke(payload, app.env.sync_task_lambda_arn.clone())
            .await?;
        println!("Sync task response: {:?}", resp);
        Ok(true.into())
    }

    async fn user_create(&self, ctx: &Context<'_>, input: UserCreateInput) -> GraphResult<Me> {
        let uid = ctx.verified_user_id()?;
        let app = ctx.data::<app::App>()?;

        let user = domain::user::User::new(uid, input.name.try_into().map_err(BadRequest.withf())?);

        let tx = app.session_manager.begin_tx().await?;
        app.user_repository.insert(&tx, user.clone()).await?;
        tx.commit().await?;

        Ok(user.into())
    }

    async fn user_update(&self, ctx: &Context<'_>, input: UserUpdateInput) -> GraphResult<Me> {
        let uid = ctx.verified_user_id()?;
        let app = ctx.data::<app::App>()?;

        let tx = app.session_manager.begin_tx().await?;
        let user = app.user_repository.get(&tx, &uid).await?;
        let user = user.update(input.name.try_into().map_err(BadRequest.withf())?);
        app.user_repository.update(&tx, user.clone()).await?;
        tx.commit().await?;

        Ok(user.into())
    }

    async fn user_delete(&self, ctx: &Context<'_>) -> GraphResult<BoolPayload> {
        let uid = ctx.verified_user_id()?;
        let app = ctx.data::<app::App>()?;

        let tx = app.session_manager.begin_tx().await?;
        let user = app.user_repository.get(&tx, &uid).await?;
        app.user_repository.delete(&tx, &user.id).await?;
        tx.commit().await?;

        Ok(true.into())
    }

    async fn order_create(&self, ctx: &Context<'_>, input: OrderCreateInput) -> GraphResult<Order> {
        let uid = ctx.verified_user_id()?;
        let app = ctx.data::<app::App>()?;

        let tx = app.session_manager.begin_tx().await?;
        let me = app.user_repository.get(&tx, &uid).await?;
        let order = domain::order::Order::new(&me);
        let details = input
            .details
            .into_iter()
            .map(|d| {
                let name: domain::order::detail::Name =
                    match d.name.try_into().map_err(BadRequest.withf()) {
                        Ok(name) => name,
                        Err(_) => {
                            return Err(BadRequest.with("Invalid product name"));
                        }
                    };
                Ok(domain::order::detail::Detail::new(&order, name, d.quantity))
            })
            .collect::<AppResult<Vec<domain::order::detail::Detail>>>()?;
        app.order_repository.insert(&tx, order.clone()).await?;
        for detail in details {
            app.order_detail_repository.insert(&tx, detail).await?;
        }
        tx.commit().await?;

        Ok(order.into())
    }
}

#[derive(InputObject)]
struct UserCreateInput {
    pub name: String,
}

#[derive(InputObject)]
struct UserUpdateInput {
    pub name: String,
}

#[derive(InputObject)]
struct OrderCreateInput {
    pub details: Vec<OrderDetailCreateInput>,
}

#[derive(InputObject)]
struct OrderDetailCreateInput {
    pub name: String,
    pub quantity: u32,
}

#[derive(InputObject)]
struct PreSignUploadInput {
    pub path: PreSignUploadPath,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Enum)]
enum PreSignUploadPath {
    Asset,
    Temp,
}

impl PreSignUploadPath {
    pub fn path_string(&self) -> String {
        match self {
            PreSignUploadPath::Asset => "asset".to_string(),
            PreSignUploadPath::Temp => "tmp".to_string(),
        }
    }
}

#[derive(SimpleObject)]
struct PreSignUploadPayload {
    pub file_id: String,
    pub key: String,
    pub url: String,
}
