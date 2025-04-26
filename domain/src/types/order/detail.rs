use crate::types;
use crate::types::order::Order;
use crate::types::time::{now, LocalDateTime};
use crate::types::{order, HasId};

pub type Id = types::Id<Detail>;
#[derive(Debug, Clone)]
pub struct Detail {
    pub id: Id,
    pub order_id: order::Id,
    pub quantity: u32,
    pub created_at: LocalDateTime,
    pub updated_at: LocalDateTime,
}
impl Detail {
    pub fn new(order: &Order, quantity: u32) -> Self {
        Self {
            id: Id::generate(),
            order_id: order.id.clone(),
            quantity,
            created_at: now(),
            updated_at: now(),
        }
    }
}
impl HasId for Detail {
    fn id(&self) -> &types::Id<Self> {
        &self.id
    }
}
