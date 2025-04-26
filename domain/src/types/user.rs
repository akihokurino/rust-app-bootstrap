use crate::types;
use crate::types::time::LocalDateTime;
use crate::types::{define_len_restricted_string_model, HasId};

pub type Id = types::Id<User>;
#[derive(Debug, Clone)]
pub struct User {
    pub id: Id,
    pub name: Name,
    pub created_at: LocalDateTime,
    pub updated_at: LocalDateTime,
}
impl HasId for User {
    fn id(&self) -> &types::Id<Self> {
        &self.id
    }
}

define_len_restricted_string_model!(Name, "ユーザー名", 1, 255);
