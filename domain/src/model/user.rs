use crate::model;
use crate::model::time::LocalDateTime;
use crate::model::{define_len_restricted_string_model, HasId};

pub type Id = model::Id<User>;
#[derive(Debug, Clone)]
pub struct User {
    pub id: Id,
    pub name: Name,
    pub created_at: LocalDateTime,
    pub updated_at: LocalDateTime,
}
impl HasId for User {
    fn id(&self) -> &model::Id<Self> {
        &self.id
    }
}

define_len_restricted_string_model!(Name, "ユーザー名", 1, 255);
