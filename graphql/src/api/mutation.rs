use crate::api::types::user::Me;
use crate::api::AppContext;
use crate::GraphResult;
use async_graphql::{Context, InputObject, MergedObject, Object};

#[derive(MergedObject, Default)]
pub struct MutationRoot(DefaultMutation);

#[derive(Default)]
pub struct DefaultMutation;
#[Object]
impl DefaultMutation {
    async fn user_create(&self, ctx: &Context<'_>, input: UserCreateInput) -> GraphResult<Me> {
        let uid = ctx.verified_user_id()?;
        unimplemented!()
    }
}

#[derive(InputObject)]
struct UserCreateInput {
    pub name: String,
}
