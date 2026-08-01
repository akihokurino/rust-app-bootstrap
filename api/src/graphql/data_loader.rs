use app::domain;

macro_rules! impl_data_loader {
    (
        $loader_name:ident,
        $data_loader_name:ident,
        $new_loader_fn:ident,
        $key_type:ty,
        $value_type:ty,
        $repository:ident
    ) => {
        pub struct $loader_name {
            app: app::App,
        }

        impl async_graphql::dataloader::Loader<$key_type> for $loader_name {
            type Value = $value_type;
            type Error = app::errors::AppError;

            async fn load(
                &self,
                keys: &[$key_type],
            ) -> Result<std::collections::HashMap<$key_type, Self::Value>, Self::Error> {
                use app::domain::IntoIdMap;
                let conn = self.app.db_session.conn();
                let ids = keys.into_iter().collect::<Vec<_>>();
                let items = self.app.$repository.get_multi(conn, ids).await?;
                Ok(items.into_id_map())
            }
        }

        pub type $data_loader_name = async_graphql::dataloader::DataLoader<$loader_name>;

        pub fn $new_loader_fn(app: app::App) -> $data_loader_name {
            async_graphql::dataloader::DataLoader::new($loader_name { app }, tokio::spawn)
        }
    };
}

macro_rules! impl_slice_data_loader {
    (
        $loader_name:ident,
        $data_loader_name:ident,
        $new_loader_fn:ident,
        $key_type:ty,
        $value_type:ty,
        $repository:ident,
        $method:ident,
        $group_key:expr
    ) => {
        pub struct $loader_name {
            app: app::App,
        }

        impl async_graphql::dataloader::Loader<$key_type> for $loader_name {
            type Value = Vec<$value_type>;
            type Error = app::errors::AppError;

            async fn load(
                &self,
                keys: &[$key_type],
            ) -> Result<std::collections::HashMap<$key_type, Self::Value>, Self::Error> {
                let conn = self.app.db_session.conn();
                let ids = keys.into_iter().collect::<Vec<_>>();
                let items = self.app.$repository.$method(conn, ids).await?;

                let group_key = $group_key;
                let mut map: std::collections::HashMap<$key_type, Self::Value> =
                    std::collections::HashMap::new();
                for item in items {
                    map.entry(group_key(&item)).or_default().push(item);
                }
                Ok(map)
            }
        }

        pub type $data_loader_name = async_graphql::dataloader::DataLoader<$loader_name>;

        pub fn $new_loader_fn(app: app::App) -> $data_loader_name {
            async_graphql::dataloader::DataLoader::new($loader_name { app }, tokio::spawn)
        }
    };
}

impl_data_loader!(
    UserLoader,
    UserDataLoader,
    new_user_loader,
    domain::user::Id,
    domain::user::User,
    user_repository
);

impl_data_loader!(
    OrderLoader,
    OrderDataLoader,
    new_order_loader,
    domain::order::Id,
    domain::order::Order,
    order_repository
);

impl_slice_data_loader!(
    OrderDetailLoader,
    OrderDetailDataLoader,
    new_order_detail_loader,
    domain::order::Id,
    domain::order::detail::Detail,
    order_detail_repository,
    get_multi_by_order,
    |v: &domain::order::detail::Detail| v.order_id.clone()
);
