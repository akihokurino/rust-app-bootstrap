pub mod detail;

use crate::types;
use crate::types::time::{now, LocalDateTime};
use crate::types::user::User;
use crate::types::{user, HasId};

pub type Id = types::Id<Order>;
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
    fn id(&self) -> &types::Id<Self> {
        &self.id
    }
}
