use crate::macros::string_model::define_len_restricted_string_model;
use crate::models;
use crate::models::order::Order;
use crate::models::time::{now, LocalDateTime};
use crate::models::{order, HasId};

pub type Id = models::Id<Detail>;
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
    fn id(&self) -> &models::Id<Self> {
        &self.id
    }
}

define_len_restricted_string_model!(Name, "商品名", 1, 255);
