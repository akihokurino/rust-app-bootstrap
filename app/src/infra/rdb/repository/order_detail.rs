use crate::adapter::DbConn;
use crate::domain::order;
use crate::domain::order::detail::{Detail, Id, OrderDetailRepository};
use crate::errors::Kind::Internal;
use crate::infra::rdb::generated::order_details;
use crate::infra::rdb::generated::prelude::*;
use crate::infra::rdb::repository;
use crate::AppResult;
use async_trait::async_trait;
use sea_orm::{entity::prelude::*, QueryFilter, QueryOrder};

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

impl From<Detail> for order_details::Model {
    fn from(v: Detail) -> Self {
        Self {
            id: v.id.into(),
            order_id: v.order_id.into(),
            product_name: v.product_name.into(),
            quantity: v.quantity as i32,
            created_at: v.created_at.into(),
            updated_at: v.updated_at.into(),
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
        repository::find::<OrderDetails, Detail, _>(db, order_details::Column::CreatedAt).await
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
        repository::get::<OrderDetails, Detail>(db, id).await
    }

    async fn get_multi(&self, db: DbConn<'_>, ids: Vec<&Id>) -> AppResult<Vec<Detail>> {
        repository::get_multi::<OrderDetails, Detail, _>(db, order_details::Column::Id, ids).await
    }

    async fn insert(&self, db: DbConn<'_>, detail: Detail) -> AppResult<()> {
        repository::insert::<OrderDetails, Detail>(db, detail).await
    }

    async fn update(&self, db: DbConn<'_>, detail: Detail) -> AppResult<()> {
        repository::update::<OrderDetails, Detail, _>(db, order_details::Column::Id, detail).await
    }

    async fn delete(&self, db: DbConn<'_>, id: &Id) -> AppResult<()> {
        repository::delete::<OrderDetails>(db, id).await
    }
}
