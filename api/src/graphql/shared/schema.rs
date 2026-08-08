use crate::graphql::data_loader;
use async_graphql::{EmptySubscription, ObjectType};

pub fn new_schema_builder<Q, M>(
    app: app::App,
    query: Q,
    mutation: M,
) -> async_graphql::SchemaBuilder<Q, M, EmptySubscription>
where
    Q: ObjectType + 'static,
    M: ObjectType + 'static,
{
    async_graphql::Schema::build(query, mutation, EmptySubscription)
        .data(app.clone())
        .data(data_loader::new_user_loader(app.clone()))
        .data(data_loader::new_order_loader(app.clone()))
        .data(data_loader::new_order_detail_loader(app.clone()))
}
