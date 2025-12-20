pub mod order;
pub mod order_detail;
pub mod user;

use crate::adapter::DbConn;
use crate::domain::HasId;
use crate::errors::Kind::{Internal, NotFound};
use crate::infra::rdb::errors::map_insert_error;
use crate::AppResult;
use sea_orm::{
    ColumnTrait, EntityTrait, Iden, IntoActiveModel, PrimaryKeyTrait, QueryFilter, QueryOrder,
};

async fn find<E, T, C>(db: DbConn<'_>, order_column: C) -> AppResult<Vec<T>>
where
    E: EntityTrait,
    T: TryFrom<E::Model, Error = String>,
    C: ColumnTrait,
{
    E::find()
        .order_by_desc(order_column)
        .all(&db)
        .await
        .map_err(Internal.from_srcf())?
        .into_iter()
        .map(|v| v.try_into().map_err(Internal.withf()))
        .collect()
}

async fn get<E, T>(db: DbConn<'_>, id: impl AsRef<str>) -> AppResult<T>
where
    E: EntityTrait,
    <E::PrimaryKey as PrimaryKeyTrait>::ValueType: From<String>,
    T: TryFrom<E::Model, Error = String>,
{
    E::find_by_id(id.as_ref().to_string())
        .one(&db)
        .await
        .map_err(Internal.from_srcf())?
        .ok_or_else(|| NotFound.default())?
        .try_into()
        .map_err(Internal.withf())
}

async fn get_multi<E, T, C>(
    db: DbConn<'_>,
    id_column: C,
    ids: Vec<impl AsRef<str>>,
) -> AppResult<Vec<T>>
where
    E: EntityTrait,
    T: TryFrom<E::Model, Error = String>,
    C: ColumnTrait,
{
    let ids: Vec<String> = ids.into_iter().map(|id| id.as_ref().to_string()).collect();
    E::find()
        .filter(id_column.is_in(ids))
        .all(&db)
        .await
        .map_err(Internal.from_srcf())?
        .into_iter()
        .map(|v| v.try_into().map_err(Internal.withf()))
        .collect()
}

async fn insert<E, T>(db: DbConn<'_>, entity: T) -> AppResult<()>
where
    E: EntityTrait,
    T: Into<E::Model>,
    E::Model: IntoActiveModel<E::ActiveModel>,
{
    let model: E::Model = entity.into();
    let active_model = model.into_active_model();
    E::insert(active_model)
        .exec(&db)
        .await
        .map_err(map_insert_error)?;
    Ok(())
}

async fn update<E, T, C>(db: DbConn<'_>, id_column: C, entity: T) -> AppResult<()>
where
    E: EntityTrait,
    T: Into<E::Model> + HasId,
    E::Model: IntoActiveModel<E::ActiveModel>,
    C: ColumnTrait,
{
    let id = entity.id().as_ref().to_string();
    let model: E::Model = entity.into();
    let active_model = model.into_active_model();
    E::update(active_model)
        .filter(id_column.eq(id))
        .exec(&db)
        .await
        .map_err(Internal.from_srcf())?;
    Ok(())
}

async fn delete<E>(db: DbConn<'_>, id: impl AsRef<str>) -> AppResult<()>
where
    E: EntityTrait,
    <E::PrimaryKey as PrimaryKeyTrait>::ValueType: From<String>,
{
    E::delete_by_id(id.as_ref().to_string())
        .exec(&db)
        .await
        .map_err(Internal.from_srcf())?;
    Ok(())
}
