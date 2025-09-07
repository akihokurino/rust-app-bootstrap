use crate::domain::order::{Id, Order};
use crate::domain::user;
use crate::errors::Kind::{Internal, NotFound};
use crate::infra::rdb::map_insert_error;
use crate::infra::rdb::types::orders;
use crate::infra::rdb::types::prelude::*;
use crate::AppResult;
use sea_orm::{entity::prelude::*, ConnectionTrait, EntityTrait, QueryFilter, QueryOrder, Set};

impl TryFrom<orders::Model> for Order {
    type Error = String;
    fn try_from(v: orders::Model) -> Result<Self, Self::Error> {
        Ok(Self {
            id: v.id.into(),
            user_id: v.user_id.into(),
            created_at: v.created_at.into(),
            updated_at: v.updated_at.into(),
        })
    }
}
impl From<Order> for orders::ActiveModel {
    fn from(v: Order) -> Self {
        Self {
            id: Set(v.id.into()),
            user_id: Set(v.user_id.into()),
            created_at: Set(v.created_at.into()),
            updated_at: Set(v.updated_at.into()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Repository {}

impl Repository {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn find<C>(&self, db: &C) -> AppResult<Vec<Order>>
    where
        C: ConnectionTrait,
    {
        Orders::find()
            .order_by_desc(orders::Column::CreatedAt)
            .all(db)
            .await
            .map_err(Internal.from_srcf())?
            .into_iter()
            .map(|v| v.try_into().map_err(Internal.withf()))
            .collect()
    }

    pub async fn find_by_user<C>(&self, db: &C, user_id: &user::Id) -> AppResult<Vec<Order>>
    where
        C: ConnectionTrait,
    {
        Orders::find()
            .filter(orders::Column::UserId.eq(user_id.as_str()))
            .order_by_desc(orders::Column::CreatedAt)
            .all(db)
            .await
            .map_err(Internal.from_srcf())?
            .into_iter()
            .map(|v| v.try_into().map_err(Internal.withf()))
            .collect()
    }

    pub async fn get<C>(&self, db: &C, id: &Id) -> AppResult<Order>
    where
        C: ConnectionTrait,
    {
        Orders::find_by_id(id.as_str())
            .one(db)
            .await
            .map_err(Internal.from_srcf())?
            .ok_or_else(|| NotFound.default())?
            .try_into()
            .map_err(Internal.withf())
    }

    pub async fn get_multi<C>(&self, db: &C, ids: Vec<&Id>) -> AppResult<Vec<Order>>
    where
        C: ConnectionTrait,
    {
        let ids: Vec<String> = ids.iter().map(|id| id.as_str().to_string()).collect();
        Orders::find()
            .filter(orders::Column::Id.is_in(ids))
            .all(db)
            .await
            .map_err(Internal.from_srcf())?
            .into_iter()
            .map(|v| v.try_into().map_err(Internal.withf()))
            .collect()
    }

    pub async fn insert<C>(&self, db: &C, order: Order) -> AppResult<()>
    where
        C: ConnectionTrait,
    {
        let active_model: orders::ActiveModel = order.into();

        Orders::insert(active_model)
            .exec(db)
            .await
            .map_err(map_insert_error)?;

        Ok(())
    }

    pub async fn update<C>(&self, db: &C, order: Order) -> AppResult<()>
    where
        C: ConnectionTrait,
    {
        let order_id = order.id.as_str().to_string();
        let active_model: orders::ActiveModel = order.into();

        Orders::update(active_model)
            .filter(orders::Column::Id.eq(order_id))
            .exec(db)
            .await
            .map_err(Internal.from_srcf())?;

        Ok(())
    }

    pub async fn delete<C>(&self, db: &C, id: &Id) -> AppResult<()>
    where
        C: ConnectionTrait,
    {
        Orders::delete_by_id(id.as_str())
            .exec(db)
            .await
            .map_err(Internal.from_srcf())?;

        Ok(())
    }
}
