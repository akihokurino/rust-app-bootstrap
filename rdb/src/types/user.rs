use crate::macros::repository::{
    impl_repository, impl_repository_delete, impl_repository_insert, impl_repository_update,
};
use crate::schema::users;
use diesel::prelude::*;
use domain::errors::Kind::Internal;
use domain::models::time::LocalDateTime;
use domain::models::user::{Id, User};
use domain::AppResult;

#[derive(Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = users)]
struct UserModel {
    pub id: String,
    pub name: String,
    pub created_at: LocalDateTime,
    pub updated_at: LocalDateTime,
}
impl TryFrom<UserModel> for User {
    type Error = String;
    fn try_from(v: UserModel) -> Result<Self, Self::Error> {
        Ok(Self {
            id: domain::models::user::Id::from(v.id),
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

impl_repository!(UserRepository, users::table, UserModel, User, Id, users::id);
impl_repository_insert!(UserRepository, users::table, UserModel, User);
impl_repository_update!(UserRepository, users::table, UserModel, User, users::id);
impl_repository_delete!(UserRepository, users::table, Id, users::id);
impl UserRepository {}
