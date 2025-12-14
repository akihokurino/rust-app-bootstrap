use crate::graphql::admin::AppContext;
use crate::graphql::GraphResult;
use async_graphql::{Context, MergedObject, Object};

#[derive(MergedObject, Default)]
pub struct QueryRoot(DefaultQuery);

#[derive(Default)]
pub struct DefaultQuery;
#[Object]
impl DefaultQuery {
    async fn me(&self, ctx: &Context<'_>) -> GraphResult<String> {
        let uid = ctx.verified_user_id()?;
        Ok(uid)
    }
}
