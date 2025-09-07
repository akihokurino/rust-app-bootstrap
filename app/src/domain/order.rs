pub mod detail;

use crate::model::time::{now, LocalDateTime};
use crate::domain::user::User;
use crate::domain::{user, HasId};

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
    fn id(&self) -> &crate::domain::Id<Self> {
        &self.id
    }
}
