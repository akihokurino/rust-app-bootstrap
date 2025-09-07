use crate::domain::user::{Id, User};
use crate::errors::Kind::{Internal, NotFound};
use crate::infra::rdb::map_insert_error;
use crate::infra::rdb::types::user;
use crate::AppResult;
use sea_orm::{entity::prelude::*, ConnectionTrait, EntityTrait, QueryFilter, QueryOrder};

#[derive(Debug, Clone)]
pub struct Repository {}
impl Repository {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn find<C>(&self, db: &C) -> AppResult<Vec<User>>
    where
        C: ConnectionTrait,
    {
        user::Entity::find()
            .order_by_desc(user::Column::CreatedAt)
            .all(db)
            .await
            .map_err(Internal.from_srcf())?
            .into_iter()
            .map(|v| v.try_into().map_err(Internal.withf()))
            .collect()
    }

    pub async fn get<C>(&self, db: &C, id: &Id) -> AppResult<User>
    where
        C: ConnectionTrait,
    {
        user::Entity::find_by_id(id.as_str())
            .one(db)
            .await
            .map_err(Internal.from_srcf())?
            .ok_or_else(|| NotFound.default())?
            .try_into()
            .map_err(Internal.withf())
    }

    pub async fn get_multi<C>(&self, db: &C, ids: Vec<&Id>) -> AppResult<Vec<User>>
    where
        C: ConnectionTrait,
    {
        let ids: Vec<String> = ids.iter().map(|id| id.as_str().to_string()).collect();
        user::Entity::find()
            .filter(user::Column::Id.is_in(ids))
            .all(db)
            .await
            .map_err(Internal.from_srcf())?
            .into_iter()
            .map(|v| v.try_into().map_err(Internal.withf()))
            .collect()
    }

    pub async fn insert<C>(&self, db: &C, user: User) -> AppResult<()>
    where
        C: ConnectionTrait,
    {
        let active_model: user::ActiveModel = user.into();

        user::Entity::insert(active_model)
            .exec(db)
            .await
            .map_err(map_insert_error)?;

        Ok(())
    }

    pub async fn update<C>(&self, db: &C, user: User) -> AppResult<()>
    where
        C: ConnectionTrait,
    {
        let user_id = user.id.as_str().to_string();
        let active_model: user::ActiveModel = user.into();

        user::Entity::update(active_model)
            .filter(user::Column::Id.eq(user_id))
            .exec(db)
            .await
            .map_err(Internal.from_srcf())?;

        Ok(())
    }

    pub async fn delete<C>(&self, db: &C, id: &Id) -> AppResult<()>
    where
        C: ConnectionTrait,
    {
        user::Entity::delete_by_id(id.as_str())
            .exec(db)
            .await
            .map_err(Internal.from_srcf())?;

        Ok(())
    }
}
