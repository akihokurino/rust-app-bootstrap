use crate::adapter::DbConn;
use crate::domain::order::Order;
use crate::domain::types::string::impl_len_restricted_string_model;
use crate::domain::types::time::{now, LocalDateTime};
use crate::domain::{order, HasId};
use crate::AppResult;
use async_trait::async_trait;

pub type Id = crate::domain::Id<Detail>;
#[derive(Debug, Clone)]
pub struct Detail {
    pub id: Id,
    pub order_id: order::Id,
    pub product_name: Name,
    pub quantity: u32,
    pub created_at: LocalDateTime,
    pub updated_at: LocalDateTime,
}
impl Detail {
    pub fn new(order: &Order, product_name: Name, quantity: u32) -> Self {
        Self {
            id: Id::generate(),
            order_id: order.id.clone(),
            product_name,
            quantity,
            created_at: now(),
            updated_at: now(),
        }
    }
}
impl HasId for Detail {
    type Entity = Self;
    fn id(&self) -> &crate::domain::Id<Self> {
        &self.id
    }
}

impl_len_restricted_string_model!(Name, "商品名", 1, 255);

#[async_trait]
pub trait OrderDetailRepository: Send + Sync {
    async fn find(&self, db: DbConn<'_>) -> AppResult<Vec<Detail>>;
    async fn find_by_order(&self, db: DbConn<'_>, order_id: &order::Id) -> AppResult<Vec<Detail>>;
    async fn get(&self, db: DbConn<'_>, id: &Id) -> AppResult<Detail>;
    async fn get_multi(&self, db: DbConn<'_>, ids: Vec<&Id>) -> AppResult<Vec<Detail>>;
    async fn insert(&self, db: DbConn<'_>, detail: Detail) -> AppResult<()>;
    async fn update(&self, db: DbConn<'_>, detail: Detail) -> AppResult<()>;
    async fn delete(&self, db: DbConn<'_>, id: &Id) -> AppResult<()>;
}
