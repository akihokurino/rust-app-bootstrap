use crate::adapter::DbConn;
use crate::domain::user::{Id, User, UserRepository};
use crate::errors::Kind::{Internal, NotFound};
use crate::infra::rdb::errors::map_insert_error;
use crate::infra::rdb::types::prelude::*;
use crate::infra::rdb::types::users;
use crate::AppResult;
use async_trait::async_trait;
use sea_orm::{entity::prelude::*, EntityTrait, QueryFilter, QueryOrder, Set};

impl TryFrom<users::Model> for User {
    type Error = String;
    fn try_from(v: users::Model) -> Result<Self, Self::Error> {
        Ok(Self {
            id: v.id.into(),
            name: v.name.try_into()?,
            created_at: v.created_at.into(),
            updated_at: v.updated_at.into(),
        })
    }
}
impl From<User> for users::ActiveModel {
    fn from(v: User) -> Self {
        Self {
            id: Set(v.id.into()),
            name: Set(v.name.into()),
            created_at: Set(v.created_at.into()),
            updated_at: Set(v.updated_at.into()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Repository;

impl Repository {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl UserRepository for Repository {
    async fn find(&self, db: DbConn<'_>) -> AppResult<Vec<User>> {
        Users::find()
            .order_by_desc(users::Column::CreatedAt)
            .all(&db)
            .await
            .map_err(Internal.from_srcf())?
            .into_iter()
            .map(|v| v.try_into().map_err(Internal.withf()))
            .collect()
    }

    async fn get(&self, db: DbConn<'_>, id: &Id) -> AppResult<User> {
        Users::find_by_id(id.as_str())
            .one(&db)
            .await
            .map_err(Internal.from_srcf())?
            .ok_or_else(|| NotFound.default())?
            .try_into()
            .map_err(Internal.withf())
    }

    async fn get_multi(&self, db: DbConn<'_>, ids: Vec<&Id>) -> AppResult<Vec<User>> {
        let ids: Vec<String> = ids.iter().map(|id| id.as_str().to_string()).collect();
        Users::find()
            .filter(users::Column::Id.is_in(ids))
            .all(&db)
            .await
            .map_err(Internal.from_srcf())?
            .into_iter()
            .map(|v| v.try_into().map_err(Internal.withf()))
            .collect()
    }

    async fn insert(&self, db: DbConn<'_>, user: User) -> AppResult<()> {
        let active_model: users::ActiveModel = user.into();

        Users::insert(active_model)
            .exec(&db)
            .await
            .map_err(map_insert_error)?;

        Ok(())
    }

    async fn update(&self, db: DbConn<'_>, user: User) -> AppResult<()> {
        let user_id = user.id.as_str().to_string();
        let active_model: users::ActiveModel = user.into();

        Users::update(active_model)
            .filter(users::Column::Id.eq(user_id))
            .exec(&db)
            .await
            .map_err(Internal.from_srcf())?;

        Ok(())
    }

    async fn delete(&self, db: DbConn<'_>, id: &Id) -> AppResult<()> {
        Users::delete_by_id(id.as_str())
            .exec(&db)
            .await
            .map_err(Internal.from_srcf())?;

        Ok(())
    }
}
