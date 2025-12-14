use crate::domain::types::email::Email;

pub type Id = crate::domain::Id<User>;
#[derive(Debug, Clone)]
pub struct User {
    pub id: Id,
    pub email: Email,
}
