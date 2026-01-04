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
