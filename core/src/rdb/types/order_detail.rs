use crate::domain::order::detail::{Detail, Id};
use crate::errors::Kind::Internal;
use crate::{domain, AppResult};
use chrono::{DateTime, Utc};
use sqlx::{FromRow, PgPool};

#[derive(FromRow)]
struct OrderDetailModel {
    pub id: String,
    pub order_id: String,
    pub product_name: String,
    pub quantity: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TryFrom<OrderDetailModel> for Detail {
    type Error = String;
    fn try_from(v: OrderDetailModel) -> Result<Self, Self::Error> {
        Ok(Self {
            id: domain::order::detail::Id::from(v.id),
            order_id: domain::order::Id::from(v.order_id),
            product_name: v.product_name.try_into()?,
            quantity: v.quantity as u32,
            created_at: v.created_at.into(),
            updated_at: v.updated_at.into(),
        })
    }
}

impl From<Detail> for OrderDetailModel {
    fn from(detail: Detail) -> Self {
        OrderDetailModel {
            id: detail.id.into(),
            order_id: detail.order_id.into(),
            product_name: detail.product_name.into(),
            quantity: detail.quantity as i32,
            created_at: detail.created_at.into(),
            updated_at: detail.updated_at.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OrderDetailRepository {}

impl OrderDetailRepository {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn find(&self, pool: &PgPool) -> AppResult<Vec<Detail>> {
        sqlx::query_as!(OrderDetailModel, "SELECT * FROM order_details")
            .fetch_all(pool)
            .await
            .map_err(Internal.from_srcf())?
            .into_iter()
            .map(|v| v.try_into().map_err(Internal.withf()))
            .collect()
    }

    pub async fn find_by_order(
        &self,
        pool: &PgPool,
        order_id: &domain::order::Id,
    ) -> AppResult<Vec<Detail>> {
        sqlx::query_as!(
            OrderDetailModel,
            "SELECT * FROM order_details WHERE order_id = $1",
            order_id.as_str()
        )
        .fetch_all(pool)
        .await
        .map_err(Internal.from_srcf())?
        .into_iter()
        .map(|v| v.try_into().map_err(Internal.withf()))
        .collect()
    }

    pub async fn get(&self, pool: &PgPool, id: &Id) -> AppResult<Detail> {
        sqlx::query_as!(
            OrderDetailModel,
            "SELECT * FROM order_details WHERE id = $1",
            id.as_str()
        )
        .fetch_one(pool)
        .await
        .map_err(Internal.from_srcf())?
        .try_into()
        .map_err(Internal.withf())
    }

    pub async fn get_multi(&self, pool: &PgPool, ids: Vec<&Id>) -> AppResult<Vec<Detail>> {
        let ids: Vec<String> = ids.iter().map(|id| id.as_str().to_string()).collect();

        sqlx::query_as!(
            OrderDetailModel,
            "SELECT * FROM order_details WHERE id = ANY($1)",
            &ids
        )
        .fetch_all(pool)
        .await
        .map_err(Internal.from_srcf())?
        .into_iter()
        .map(|v| v.try_into().map_err(Internal.withf()))
        .collect()
    }

    pub async fn insert(&self, pool: &PgPool, entity: Detail) -> AppResult<()> {
        let model: OrderDetailModel = entity.into();

        sqlx::query!(
            "
INSERT INTO order_details 
    (id, order_id, product_name, quantity, created_at, updated_at) 
VALUES 
    ($1, $2, $3, $4, $5, $6)",
            model.id,
            model.order_id,
            model.product_name,
            model.quantity,
            model.created_at,
            model.updated_at
        )
        .execute(pool)
        .await
        .map_err(Internal.from_srcf())?;

        Ok(())
    }

    pub async fn update(&self, pool: &PgPool, entity: Detail) -> AppResult<()> {
        let model: OrderDetailModel = entity.into();

        sqlx::query!(
            "
UPDATE order_details 
SET 
    order_id = $2, 
    product_name = $3, 
    quantity = $4, 
    created_at = $5, 
    updated_at = $6 
WHERE id = $1",
            model.id,
            model.order_id,
            model.product_name,
            model.quantity,
            model.created_at,
            model.updated_at
        )
        .execute(pool)
        .await
        .map_err(Internal.from_srcf())?;

        Ok(())
    }

    pub async fn delete(&self, pool: &PgPool, id: &Id) -> AppResult<()> {
        sqlx::query!("DELETE FROM order_details WHERE id = $1", id.as_str())
            .execute(pool)
            .await
            .map_err(Internal.from_srcf())?;

        Ok(())
    }
}
