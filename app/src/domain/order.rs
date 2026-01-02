pub mod detail;

use crate::adapter::DbConn;
use crate::domain::types::time::{now, LocalDateTime};
use crate::domain::user::User;
use crate::domain::{user, HasId};
use crate::AppResult;
use async_trait::async_trait;

pub type Id = crate::domain::Id<Order>;
#[derive(Debug, Clone)]
pub struct Order {
    pub id: Id,
    pub user_id: user::Id,
    pub created_at: LocalDateTime,
    pub updated_at: LocalDateTime,
}
impl Order {
    pub fn new(user: &User) -> Self {
        Self {
            id: Id::generate(),
            user_id: user.id.clone(),
            created_at: now(),
            updated_at: now(),
        }
    }
}
impl HasId for Order {
    type Entity = Self;
    fn id(&self) -> &crate::domain::Id<Self> {
        &self.id
    }
}

#[async_trait]
pub trait OrderRepository: Send + Sync {
    async fn find(&self, db: DbConn<'_>) -> AppResult<Vec<Order>>;
    async fn find_by_user(&self, db: DbConn<'_>, user_id: &user::Id) -> AppResult<Vec<Order>>;
    async fn get(&self, db: DbConn<'_>, id: &Id) -> AppResult<Order>;
    async fn get_multi(&self, db: DbConn<'_>, ids: Vec<&Id>) -> AppResult<Vec<Order>>;
    async fn insert(&self, db: DbConn<'_>, order: Order) -> AppResult<()>;
    async fn update(&self, db: DbConn<'_>, order: Order) -> AppResult<()>;
    async fn delete(&self, db: DbConn<'_>, id: &Id) -> AppResult<()>;
}
