use crate::api::types::user::{Me, User};
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

        let rdb_resolver = ctx.data::<rdb::Resolver>()?;
        let user = rdb_resolver.session_manager.read(|conn| {
            let user_repository = &rdb_resolver.user_repository;
            user_repository.get(conn, &uid)
        })?;

        Ok(user.into())
    }

    async fn users(&self, ctx: &Context<'_>) -> GraphResult<Vec<User>> {
        let rdb_resolver = ctx.data::<rdb::Resolver>()?;

        let users = rdb_resolver.session_manager.read(|conn| {
            let user_repository = &rdb_resolver.user_repository;
            user_repository.find(conn)
        })?;

        Ok(users.into_iter().map(|v| v.into()).collect())
    }
}
