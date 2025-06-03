use crate::graphql::service::types::order::Order;
use crate::graphql::service::types::user::Me;
use crate::graphql::shared::types::BoolPayload;
use crate::graphql::GraphResult;
use async_graphql::futures_util::FutureExt;
use async_graphql::{Context, InputObject, MergedObject, Object};

#[derive(MergedObject, Default)]
pub struct MutationRoot(DefaultMutation);

#[derive(Default)]
pub struct DefaultMutation;
#[Object]
impl DefaultMutation {
    async fn user_create(&self, ctx: &Context<'_>, input: UserCreateInput) -> GraphResult<Me> {
        // let uid = ctx.verified_user_id()?;
        // let core_resolver = ctx.data::<core::Resolver>()?;
        // let user_repo = &core_resolver.user_repository;
        //
        // let user = domain::user::User::new(uid, input.name.try_into().map_err(BadRequest.withf())?);
        // core_resolver
        //     .session_manager
        //     .transaction(|tx| {
        //         async move { core_resolver.user_repository.insert(tx, user.clone()).await }.boxed()
        //     })
        //     .await?;
        //
        // Ok(user.into())
        panic!()
    }

    async fn user_update(&self, ctx: &Context<'_>, input: UserUpdateInput) -> GraphResult<Me> {
        // let uid = ctx.verified_user_id()?;
        // let core_resolver = ctx.data::<core::Resolver>()?;
        //
        // let user = core_resolver.session_manager.transaction(|conn| async {
        //     let user = core_resolver.user_repository.get(conn, &uid).await?;
        //     let user = user.update(input.name.try_into().map_err(BadRequest.withf())?);
        //     core_resolver
        //         .user_repository
        //         .update(conn, user.clone())
        //         .await?;
        //     Ok(user)
        // })?;
        //
        // Ok(user.into())
        panic!()
    }

    async fn user_delete(&self, ctx: &Context<'_>) -> GraphResult<BoolPayload> {
        // let uid = ctx.verified_user_id()?;
        // let core_resolver = ctx.data::<core::Resolver>()?;
        //
        // core_resolver.session_manager.transaction(|conn| {
        //     let user = core_resolver.user_repository.get(conn, &uid)?;
        //     core_resolver.user_repository.delete(conn, &user.id)
        // })?;
        //
        // Ok(true.into())
        panic!()
    }

    async fn order_create(&self, ctx: &Context<'_>, input: OrderCreateInput) -> GraphResult<Order> {
        // let uid = ctx.verified_user_id()?;
        // let core_resolver = ctx.data::<core::Resolver>()?;
        //
        // let pool = core_resolver.session_manager.pool();
        // let me = core_resolver.user_repository.get(pool, &uid).await?;
        // let order = domain::order::Order::new(&me);
        // let details = input
        //     .details
        //     .into_iter()
        //     .map(|d| {
        //         let name: domain::order::detail::Name =
        //             match d.name.try_into().map_err(BadRequest.withf()) {
        //                 Ok(name) => name,
        //                 Err(_) => {
        //                     return Err(BadRequest.with("Invalid product name"));
        //                 }
        //             };
        //         Ok(domain::order::detail::Detail::new(&order, name, d.quantity))
        //     })
        //     .collect::<AppResult<Vec<domain::order::detail::Detail>>>()?;
        //
        // core_resolver.session_manager.transaction(|conn| {
        //     core_resolver.order_repository.insert(conn, order.clone())?;
        //     for detail in details {
        //         core_resolver.order_detail_repository.insert(conn, detail)?;
        //     }
        //     Ok(())
        // })?;
        //
        // Ok(order.into())
        panic!()
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
