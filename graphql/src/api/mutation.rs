use crate::api::types::order::Order;
use crate::api::types::user::Me;
use crate::api::AppContext;
use crate::shared::types::BoolPayload;
use crate::GraphResult;
use async_graphql::{Context, InputObject, MergedObject, Object};
use domain::errors::Kind::BadRequest;
use domain::AppResult;

#[derive(MergedObject, Default)]
pub struct MutationRoot(DefaultMutation);

#[derive(Default)]
pub struct DefaultMutation;
#[Object]
impl DefaultMutation {
    async fn user_create(&self, ctx: &Context<'_>, input: UserCreateInput) -> GraphResult<Me> {
        let uid = ctx.verified_user_id()?;
        let rdb_resolver = ctx.data::<rdb::Resolver>()?;

        let user = domain::models::user::User::new(
            uid,
            input.name.try_into().map_err(BadRequest.withf())?,
        );
        rdb_resolver.session_manager.transaction(|conn| {
            let user_repository = &rdb_resolver.user_repository;
            user_repository.insert(conn, user.clone())
        })?;

        Ok(user.into())
    }

    async fn user_update(&self, ctx: &Context<'_>, input: UserUpdateInput) -> GraphResult<Me> {
        let uid = ctx.verified_user_id()?;
        let rdb_resolver = ctx.data::<rdb::Resolver>()?;

        let user = rdb_resolver.session_manager.transaction(|conn| {
            let user_repository = &rdb_resolver.user_repository;
            let user = user_repository.get(conn, &uid)?;
            let user = user.update(input.name.try_into().map_err(BadRequest.withf())?);
            user_repository.update(conn, user.clone())?;
            Ok(user)
        })?;

        Ok(user.into())
    }

    async fn user_delete(&self, ctx: &Context<'_>) -> GraphResult<BoolPayload> {
        let uid = ctx.verified_user_id()?;
        let rdb_resolver = ctx.data::<rdb::Resolver>()?;

        rdb_resolver.session_manager.transaction(|conn| {
            let user_repository = &rdb_resolver.user_repository;
            let user = user_repository.get(conn, &uid)?;
            user_repository.delete(conn, &user.id)
        })?;

        Ok(true.into())
    }

    async fn order_create(&self, ctx: &Context<'_>, input: OrderCreateInput) -> GraphResult<Order> {
        let uid = ctx.verified_user_id()?;
        let rdb_resolver = ctx.data::<rdb::Resolver>()?;

        let me = rdb_resolver
            .session_manager
            .read(|conn| rdb_resolver.user_repository.get(conn, &uid))?;
        let order = domain::models::order::Order::new(&me);
        let details = input
            .details
            .into_iter()
            .map(|d| {
                let name: domain::models::order::detail::Name =
                    match d.name.try_into().map_err(BadRequest.withf()) {
                        Ok(name) => name,
                        Err(_) => {
                            return Err(BadRequest.with("Invalid product name"));
                        }
                    };
                Ok(domain::models::order::detail::Detail::new(
                    &order, name, d.quantity,
                ))
            })
            .collect::<AppResult<Vec<domain::models::order::detail::Detail>>>()?;

        rdb_resolver.session_manager.transaction(|conn| {
            rdb_resolver.order_repository.insert(conn, order.clone())?;
            for detail in details {
                rdb_resolver.order_detail_repository.insert(conn, detail)?;
            }
            Ok(())
        })?;

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
