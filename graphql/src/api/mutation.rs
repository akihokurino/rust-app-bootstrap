use crate::api::types::order::Order;
use crate::api::types::user::Me;
use crate::api::AppContext;
use crate::GraphResult;
use async_graphql::{Context, InputObject, MergedObject, Object};
use domain::errors::Kind::BadRequest;

#[derive(MergedObject, Default)]
pub struct MutationRoot(DefaultMutation);

#[derive(Default)]
pub struct DefaultMutation;
#[Object]
impl DefaultMutation {
    async fn user_create(&self, ctx: &Context<'_>, input: UserCreateInput) -> GraphResult<Me> {
        let uid = ctx.verified_user_id()?;

        // TODO: implement
        let user = domain::models::user::User {
            id: uid.clone(),
            name: input.name.try_into().map_err(BadRequest.withf())?,
            created_at: domain::models::time::now(),
            updated_at: domain::models::time::now(),
        };
        Ok(user.into())
    }

    async fn order_create(
        &self,
        ctx: &Context<'_>,
        _input: OrderCreateInput,
    ) -> GraphResult<Order> {
        let uid = ctx.verified_user_id()?;

        // TODO: implement
        let order = domain::models::order::Order {
            id: domain::models::order::Id::generate(),
            user_id: uid.clone(),
            created_at: domain::models::time::now(),
            updated_at: domain::models::time::now(),
        };
        Ok(order.into())
    }
}

#[derive(InputObject)]
struct UserCreateInput {
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
