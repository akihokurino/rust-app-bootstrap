use crate::domain::order::Id;
use crate::errors::Kind::{Internal, NotFound};
use crate::infra::rdb::map_insert_error;
use crate::infra::rdb::types::order;
use crate::{domain, AppResult};
use sea_orm::{entity::prelude::*, ConnectionTrait, EntityTrait, QueryFilter, QueryOrder};

#[derive(Debug, Clone)]
pub struct Repository {}

impl Repository {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn find<C>(&self, db: &C) -> AppResult<Vec<domain::order::Order>>
    where
        C: ConnectionTrait,
    {
        order::Entity::find()
            .order_by_desc(order::Column::CreatedAt)
            .all(db)
            .await
            .map_err(Internal.from_srcf())?
            .into_iter()
            .map(|v| v.try_into().map_err(Internal.withf()))
            .collect()
    }

    pub async fn find_by_user<C>(
        &self,
        db: &C,
        user_id: &domain::user::Id,
    ) -> AppResult<Vec<domain::order::Order>>
    where
        C: ConnectionTrait,
    {
        order::Entity::find()
            .filter(order::Column::UserId.eq(user_id.as_str()))
            .order_by_desc(order::Column::CreatedAt)
            .all(db)
            .await
            .map_err(Internal.from_srcf())?
            .into_iter()
            .map(|v| v.try_into().map_err(Internal.withf()))
            .collect()
    }

    pub async fn get<C>(&self, db: &C, id: &Id) -> AppResult<domain::order::Order>
    where
        C: ConnectionTrait,
    {
        order::Entity::find_by_id(id.as_str())
            .one(db)
            .await
            .map_err(Internal.from_srcf())?
            .ok_or_else(|| NotFound.default())?
            .try_into()
            .map_err(Internal.withf())
    }

    pub async fn get_multi<C>(&self, db: &C, ids: Vec<&Id>) -> AppResult<Vec<domain::order::Order>>
    where
        C: ConnectionTrait,
    {
        let ids: Vec<String> = ids.iter().map(|id| id.as_str().to_string()).collect();
        order::Entity::find()
            .filter(order::Column::Id.is_in(ids))
            .all(db)
            .await
            .map_err(Internal.from_srcf())?
            .into_iter()
            .map(|v| v.try_into().map_err(Internal.withf()))
            .collect()
    }

    pub async fn insert<C>(&self, db: &C, order: domain::order::Order) -> AppResult<()>
    where
        C: ConnectionTrait,
    {
        let active_model: order::ActiveModel = order.into();

        order::Entity::insert(active_model)
            .exec(db)
            .await
            .map_err(map_insert_error)?;

        Ok(())
    }

    pub async fn update<C>(&self, db: &C, order: domain::order::Order) -> AppResult<()>
    where
        C: ConnectionTrait,
    {
        let order_id = order.id.as_str().to_string();
        let active_model: order::ActiveModel = order.into();

        order::Entity::update(active_model)
            .filter(order::Column::Id.eq(order_id))
            .exec(db)
            .await
            .map_err(Internal.from_srcf())?;

        Ok(())
    }

    pub async fn delete<C>(&self, db: &C, id: &Id) -> AppResult<()>
    where
        C: ConnectionTrait,
    {
        order::Entity::delete_by_id(id.as_str())
            .exec(db)
            .await
            .map_err(Internal.from_srcf())?;

        Ok(())
    }
}
