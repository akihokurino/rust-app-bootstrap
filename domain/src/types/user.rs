use crate::macros::string_model::define_len_restricted_string_model;
use crate::types;
use crate::types::time::{now, LocalDateTime};
use crate::types::HasId;

pub type Id = types::Id<User>;
#[derive(Debug, Clone)]
pub struct User {
    pub id: Id,
    pub name: Name,
    pub created_at: LocalDateTime,
    pub updated_at: LocalDateTime,
}
impl User {
    pub fn new(id: Id, name: Name) -> Self {
        Self {
            id,
            name,
            created_at: now(),
            updated_at: now(),
        }
    }
}
impl HasId for User {
    fn id(&self) -> &types::Id<Self> {
        &self.id
    }
}

define_len_restricted_string_model!(Name, "ユーザー名", 1, 255);
