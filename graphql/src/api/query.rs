use crate::api::types::user::Me;
use crate::api::AppContext;
use crate::GraphResult;
use async_graphql::{Context, MergedObject, Object};

#[derive(MergedObject, Default)]
pub struct QueryRoot(DefaultQuery);

#[derive(Default)]
pub struct DefaultQuery;
#[Object]
impl DefaultQuery {
    async fn me(&self, ctx: &Context<'_>) -> GraphResult<Me> {
        let uid = ctx.verified_user_id()?;

        // TODO: implement
        let user = domain::models::user::User {
            id: uid.clone(),
            name: domain::models::user::Name::try_from("sample".to_string()).unwrap(),
            created_at: domain::models::time::now(),
            updated_at: domain::models::time::now(),
        };
        Ok(user.into())
    }
}
