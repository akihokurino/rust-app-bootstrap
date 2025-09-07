use crate::model::string::impl_len_restricted_string_model;
use crate::model::time::{now, LocalDateTime};
use crate::domain::HasId;

pub type Id = crate::domain::Id<User>;
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

    pub fn update(self, name: Name) -> Self {
        Self {
            name,
            updated_at: now(),
            ..self
        }
    }
}
impl HasId for User {
    fn id(&self) -> &crate::domain::Id<Self> {
        &self.id
    }
}

impl_len_restricted_string_model!(Name, "ユーザー名", 1, 255);
