pub mod detail;

use crate::models;
use crate::models::time::{now, LocalDateTime};
use crate::models::user::User;
use crate::models::{user, HasId};

pub type Id = models::Id<Order>;
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
    fn id(&self) -> &models::Id<Self> {
        &self.id
    }
}
