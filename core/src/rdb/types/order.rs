use crate::domain::order::{Id, Order};
use crate::errors::Kind::Internal;
use crate::{domain, AppResult};
use chrono::{DateTime, Utc};
use sqlx::{FromRow, PgPool};

#[derive(FromRow)]
struct OrderModel {
    pub id: String,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
impl TryFrom<OrderModel> for Order {
    type Error = String;
    fn try_from(v: OrderModel) -> Result<Self, Self::Error> {
        Ok(Self {
            id: domain::order::Id::from(v.id),
            user_id: domain::user::Id::from(v.user_id),
            created_at: v.created_at.into(),
            updated_at: v.updated_at.into(),
        })
    }
}
impl From<Order> for OrderModel {
    fn from(order: Order) -> Self {
        OrderModel {
            id: order.id.into(),
            user_id: order.user_id.into(),
            created_at: order.created_at.into(),
            updated_at: order.updated_at.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OrderRepository {}

impl OrderRepository {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn find(&self, pool: &PgPool) -> AppResult<Vec<Order>> {
        sqlx::query_as!(OrderModel, "SELECT * FROM orders")
            .fetch_all(pool)
            .await
            .map_err(Internal.from_srcf())?
            .into_iter()
            .map(|v| v.try_into().map_err(Internal.withf()))
            .collect()
    }

    pub async fn find_by_user(
        &self,
        pool: &PgPool,
        user_id: &domain::user::Id,
    ) -> AppResult<Vec<Order>> {
        sqlx::query_as!(
            OrderModel,
            "SELECT * FROM orders WHERE user_id = $1",
            user_id.as_str()
        )
        .fetch_all(pool)
        .await
        .map_err(Internal.from_srcf())?
        .into_iter()
        .map(|v| v.try_into().map_err(Internal.withf()))
        .collect()
    }

    pub async fn get(&self, pool: &PgPool, id: &Id) -> AppResult<Order> {
        sqlx::query_as!(
            OrderModel,
            "SELECT * FROM orders WHERE id = $1",
            id.as_str()
        )
        .fetch_one(pool)
        .await
        .map_err(Internal.from_srcf())?
        .try_into()
        .map_err(Internal.withf())
    }

    pub async fn get_multi(&self, pool: &PgPool, ids: Vec<&Id>) -> AppResult<Vec<Order>> {
        let ids: Vec<String> = ids.iter().map(|id| id.as_str().to_string()).collect();

        sqlx::query_as!(OrderModel, "SELECT * FROM orders WHERE id = ANY($1)", &ids)
            .fetch_all(pool)
            .await
            .map_err(Internal.from_srcf())?
            .into_iter()
            .map(|v| v.try_into().map_err(Internal.withf()))
            .collect()
    }

    pub async fn insert(&self, pool: &PgPool, entity: Order) -> AppResult<()> {
        let model: OrderModel = entity.into();

        sqlx::query!(
            "
INSERT INTO orders 
    (id, user_id, created_at, updated_at) 
VALUES 
    ($1, $2, $3, $4)",
            model.id,
            model.user_id,
            model.created_at,
            model.updated_at
        )
        .execute(pool)
        .await
        .map_err(Internal.from_srcf())?;

        Ok(())
    }

    pub async fn update(&self, pool: &PgPool, entity: Order) -> AppResult<()> {
        let model: OrderModel = entity.into();

        sqlx::query!(
            "
UPDATE orders 
SET 
    user_id = $2, 
    created_at = $3, 
    updated_at = $4 
WHERE id = $1",
            model.id,
            model.user_id,
            model.created_at,
            model.updated_at
        )
        .execute(pool)
        .await
        .map_err(Internal.from_srcf())?;

        Ok(())
    }

    pub async fn delete(&self, pool: &PgPool, id: &Id) -> AppResult<()> {
        sqlx::query!("DELETE FROM orders WHERE id = $1", id.as_str())
            .execute(pool)
            .await
            .map_err(Internal.from_srcf())?;

        Ok(())
    }
}
