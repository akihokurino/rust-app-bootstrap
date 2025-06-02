use crate::domain::order::Order;
use crate::domain::string::impl_len_restricted_string_model;
use crate::domain::time::{now, LocalDateTime};
use crate::domain::{order, HasId};

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
    fn id(&self) -> &crate::domain::Id<Self> {
        &self.id
    }
}

impl_len_restricted_string_model!(Name, "商品名", 1, 255);
