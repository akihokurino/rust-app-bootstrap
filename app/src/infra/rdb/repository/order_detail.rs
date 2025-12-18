use crate::adapter::DbConn;
use crate::domain::order;
use crate::domain::order::detail::{Detail, Id, OrderDetailRepository};
use crate::errors::Kind::{Internal, NotFound};
use crate::infra::rdb::errors::map_insert_error;
use crate::infra::rdb::types::order_details;
use crate::infra::rdb::types::prelude::*;
use crate::AppResult;
use async_trait::async_trait;
use sea_orm::{entity::prelude::*, EntityTrait, QueryFilter, QueryOrder, Set};

impl TryFrom<order_details::Model> for Detail {
    type Error = String;
    fn try_from(v: order_details::Model) -> Result<Self, Self::Error> {
        Ok(Self {
            id: v.id.into(),
            order_id: v.order_id.into(),
            product_name: v.product_name.try_into()?,
            quantity: v.quantity as u32,
            created_at: v.created_at.into(),
            updated_at: v.updated_at.into(),
        })
    }
}
impl From<Detail> for order_details::ActiveModel {
    fn from(v: Detail) -> Self {
        Self {
            id: Set(v.id.into()),
            order_id: Set(v.order_id.into()),
            product_name: Set(v.product_name.into()),
            quantity: Set(v.quantity as i32),
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
impl OrderDetailRepository for Repository {
    async fn find(&self, db: DbConn<'_>) -> AppResult<Vec<Detail>> {
        OrderDetails::find()
            .order_by_desc(order_details::Column::CreatedAt)
            .all(&db)
            .await
            .map_err(Internal.from_srcf())?
            .into_iter()
            .map(|v| v.try_into().map_err(Internal.withf()))
            .collect()
    }

    async fn find_by_order(&self, db: DbConn<'_>, order_id: &order::Id) -> AppResult<Vec<Detail>> {
        OrderDetails::find()
            .filter(order_details::Column::OrderId.eq(order_id.as_str()))
            .order_by_desc(order_details::Column::CreatedAt)
            .all(&db)
            .await
            .map_err(Internal.from_srcf())?
            .into_iter()
            .map(|v| v.try_into().map_err(Internal.withf()))
            .collect()
    }

    async fn get(&self, db: DbConn<'_>, id: &Id) -> AppResult<Detail> {
        OrderDetails::find_by_id(id.as_str())
            .one(&db)
            .await
            .map_err(Internal.from_srcf())?
            .ok_or_else(|| NotFound.default())?
            .try_into()
            .map_err(Internal.withf())
    }

    async fn get_multi(&self, db: DbConn<'_>, ids: Vec<&Id>) -> AppResult<Vec<Detail>> {
        let ids: Vec<String> = ids.iter().map(|id| id.as_str().to_string()).collect();
        OrderDetails::find()
            .filter(order_details::Column::Id.is_in(ids))
            .all(&db)
            .await
            .map_err(Internal.from_srcf())?
            .into_iter()
            .map(|v| v.try_into().map_err(Internal.withf()))
            .collect()
    }

    async fn insert(&self, db: DbConn<'_>, detail: Detail) -> AppResult<()> {
        let active_model: order_details::ActiveModel = detail.into();

        OrderDetails::insert(active_model)
            .exec(&db)
            .await
            .map_err(map_insert_error)?;

        Ok(())
    }

    async fn update(&self, db: DbConn<'_>, detail: Detail) -> AppResult<()> {
        let detail_id = detail.id.as_str().to_string();
        let active_model: order_details::ActiveModel = detail.into();

        OrderDetails::update(active_model)
            .filter(order_details::Column::Id.eq(detail_id))
            .exec(&db)
            .await
            .map_err(Internal.from_srcf())?;

        Ok(())
    }

    async fn delete(&self, db: DbConn<'_>, id: &Id) -> AppResult<()> {
        OrderDetails::delete_by_id(id.as_str())
            .exec(&db)
            .await
            .map_err(Internal.from_srcf())?;

        Ok(())
    }
}
