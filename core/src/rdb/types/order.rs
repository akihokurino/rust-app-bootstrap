use crate::domain::order::{Id, Order};
use crate::errors::Kind::Internal;
use crate::{domain, AppResult};
use chrono::{DateTime, Utc};
use sqlx::{FromRow, Postgres};

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

    pub async fn find<'a, E>(&self, pool: E) -> AppResult<Vec<Order>>
    where
        E: sqlx::Executor<'a, Database = Postgres>,
    {
        sqlx::query_as!(OrderModel, "SELECT * FROM orders")
            .fetch_all(pool)
            .await
            .map_err(Internal.from_srcf())?
            .into_iter()
            .map(|v| v.try_into().map_err(Internal.withf()))
            .collect()
    }

    pub async fn find_by_user<'a, E>(
        &self,
        pool: E,
        user_id: &domain::user::Id,
    ) -> AppResult<Vec<Order>>
    where
        E: sqlx::Executor<'a, Database = Postgres>,
    {
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

    pub async fn get<'a, E>(&self, pool: E, id: &Id) -> AppResult<Order>
    where
        E: sqlx::Executor<'a, Database = Postgres>,
    {
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

    pub async fn get_multi<'a, E>(&self, pool: E, ids: Vec<&Id>) -> AppResult<Vec<Order>>
    where
        E: sqlx::Executor<'a, Database = Postgres>,
    {
        let ids: Vec<String> = ids.iter().map(|id| id.as_str().to_string()).collect();

        sqlx::query_as!(OrderModel, "SELECT * FROM orders WHERE id = ANY($1)", &ids)
            .fetch_all(pool)
            .await
            .map_err(Internal.from_srcf())?
            .into_iter()
            .map(|v| v.try_into().map_err(Internal.withf()))
            .collect()
    }

    pub async fn insert<'a, E>(&self, pool: E, entity: Order) -> AppResult<()>
    where
        E: sqlx::Executor<'a, Database = Postgres>,
    {
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

    pub async fn update<'a, E>(&self, pool: E, entity: Order) -> AppResult<()>
    where
        E: sqlx::Executor<'a, Database = Postgres>,
    {
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

    pub async fn delete<'a, E>(&self, pool: E, id: &Id) -> AppResult<()>
    where
        E: sqlx::Executor<'a, Database = Postgres>,
    {
        sqlx::query!("DELETE FROM orders WHERE id = $1", id.as_str())
            .execute(pool)
            .await
            .map_err(Internal.from_srcf())?;

        Ok(())
    }
}
