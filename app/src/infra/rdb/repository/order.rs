use crate::adapter::DbConn;
use crate::domain::order::{Id, Order, OrderRepository};
use crate::domain::user;
use crate::errors::Kind::Internal;
use crate::infra::rdb::generated::orders;
use crate::infra::rdb::generated::prelude::*;
use crate::infra::rdb::repository;
use crate::AppResult;
use async_trait::async_trait;
use sea_orm::{entity::prelude::*, QueryFilter, QueryOrder};

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

impl From<Order> for orders::Model {
    fn from(v: Order) -> Self {
        Self {
            id: v.id.into(),
            user_id: v.user_id.into(),
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
impl OrderRepository for Repository {
    async fn find(&self, db: DbConn<'_>) -> AppResult<Vec<Order>> {
        repository::find::<Orders, Order, _>(db, orders::Column::CreatedAt).await
    }

    async fn find_by_user(&self, db: DbConn<'_>, user_id: &user::Id) -> AppResult<Vec<Order>> {
        Orders::find()
            .filter(orders::Column::UserId.eq(user_id.as_str()))
            .order_by_desc(orders::Column::CreatedAt)
            .all(&db)
            .await
            .map_err(Internal.from_srcf())?
            .into_iter()
            .map(|v| v.try_into().map_err(Internal.withf()))
            .collect()
    }

    async fn get(&self, db: DbConn<'_>, id: &Id) -> AppResult<Order> {
        repository::get::<Orders, Order>(db, id).await
    }

    async fn get_multi(&self, db: DbConn<'_>, ids: Vec<&Id>) -> AppResult<Vec<Order>> {
        repository::get_multi::<Orders, Order, _>(db, orders::Column::Id, ids).await
    }

    async fn insert(&self, db: DbConn<'_>, order: Order) -> AppResult<()> {
        repository::insert::<Orders, Order>(db, order).await
    }

    async fn update(&self, db: DbConn<'_>, order: Order) -> AppResult<()> {
        repository::update::<Orders, Order, _>(db, orders::Column::Id, order).await
    }

    async fn delete(&self, db: DbConn<'_>, id: &Id) -> AppResult<()> {
        repository::delete::<Orders>(db, id).await
    }
}
