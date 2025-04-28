use crate::schema::users;
use diesel::prelude::*;
use domain::types::time::LocalDateTime;
use domain::types::user::User;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = users)]
pub struct UserModel {
    pub id: String,
    pub name: String,
    pub created_at: LocalDateTime,
    pub updated_at: LocalDateTime,
}
impl TryFrom<UserModel> for User {
    type Error = String;
    fn try_from(v: UserModel) -> Result<Self, Self::Error> {
        Ok(Self {
            id: domain::types::user::Id::from(v.id.clone()),
            name: v.name.try_into()?,
            created_at: v.created_at,
            updated_at: v.updated_at,
        })
    }
}
impl Into<UserModel> for User {
    fn into(self) -> UserModel {
        UserModel {
            id: self.id.into(),
            name: self.name.into(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
