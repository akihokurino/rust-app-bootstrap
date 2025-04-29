macro_rules! impl_repository {
    ($repo_name:ident, $table:path, $model:ty, $entity:ty, $id_type:ty, $id_column:path) => {
        #[derive(Debug, Clone)]
        pub struct $repo_name {}

        impl $repo_name {
            pub fn new() -> Self {
                Self {}
            }

            pub fn find(&self, conn: &mut PgConnection) -> AppResult<Vec<$entity>> {
                let items = $table.load::<$model>(conn).map_err(Internal.from_srcf())?;

                items
                    .into_iter()
                    .map(|v| v.try_into().map_err(|v| Internal.with(v)))
                    .collect::<AppResult<Vec<$entity>>>()
            }

            pub fn get(&self, conn: &mut PgConnection, id: &$id_type) -> AppResult<$entity> {
                let item = $table
                    .filter($id_column.eq(id.as_str()))
                    .first::<$model>(conn)
                    .map_err(Internal.from_srcf())?;

                item.try_into().map_err(|v| Internal.with(v))
            }
        }
    };
}
pub(crate) use impl_repository;

macro_rules! impl_repository_insert {
    ($repo_name:ident, $table:path, $model:ty, $entity:ty) => {
        impl $repo_name {
            pub fn insert(&self, conn: &mut PgConnection, entity: $entity) -> AppResult<()> {
                let model: $model = entity.into();
                diesel::insert_into($table)
                    .values(&model)
                    .execute(conn)
                    .map_err(Internal.from_srcf())?;
                Ok(())
            }
        }
    };
}
pub(crate) use impl_repository_insert;

macro_rules! impl_repository_update {
    ($repo_name:ident, $table:path, $model:ty, $entity:ty, $id_column:path) => {
        impl $repo_name {
            pub fn update(&self, conn: &mut PgConnection, entity: $entity) -> AppResult<()> {
                let model: $model = entity.into();
                diesel::update($table.filter($id_column.eq(model.id.clone())))
                    .set(&model)
                    .execute(conn)
                    .map_err(Internal.from_srcf())?;
                Ok(())
            }
        }
    };
}
pub(crate) use impl_repository_update;

macro_rules! impl_repository_delete {
    ($repo_name:ident, $table:path, $id_type:ty, $id_column:path) => {
        impl $repo_name {
            pub fn delete(&self, conn: &mut PgConnection, id: &$id_type) -> AppResult<()> {
                diesel::delete($table.filter($id_column.eq(id.as_str())))
                    .execute(conn)
                    .map_err(Internal.from_srcf())?;
                Ok(())
            }
        }
    };
}
pub(crate) use impl_repository_delete;
