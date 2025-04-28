use crate::schema::users;
use diesel::prelude::*;
use domain::errors::Kind::Internal;
use domain::models::time::LocalDateTime;
use domain::models::user::{Id, User};
use domain::AppResult;

#[derive(Queryable, Selectable, Insertable, AsChangeset)]
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
            id: domain::models::user::Id::from(v.id.clone()),
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

#[derive(Debug, Clone)]
pub struct UserRepository {}
impl UserRepository {
    pub fn find(&self, conn: &mut PgConnection) -> AppResult<Vec<User>> {
        let users = users::table
            .load::<UserModel>(conn)
            .map_err(Internal.from_srcf())?;
        users
            .into_iter()
            .map(|v| v.try_into().map_err(|v| Internal.with(v)))
            .collect::<AppResult<Vec<User>>>()
    }

    pub fn get(&self, conn: &mut PgConnection, id: &Id) -> AppResult<User> {
        let user = users::table
            .filter(users::id.eq(id.as_str()))
            .first::<UserModel>(conn)
            .map_err(Internal.from_srcf())?;
        user.try_into().map_err(|v| Internal.with(v))
    }

    pub fn insert(&self, conn: &mut PgConnection, user: User) -> AppResult<()> {
        let user: UserModel = user.into();
        diesel::insert_into(users::table)
            .values(&user)
            .execute(conn)
            .map_err(Internal.from_srcf())?;
        Ok(())
    }

    pub fn update(&self, conn: &mut PgConnection, user: User) -> AppResult<()> {
        let user: UserModel = user.into();
        diesel::update(users::table.filter(users::id.eq(user.id.clone())))
            .set(&user)
            .execute(conn)
            .map_err(Internal.from_srcf())?;
        Ok(())
    }

    pub fn delete(&self, conn: &mut PgConnection, id: &Id) -> AppResult<()> {
        diesel::delete(users::table.filter(users::id.eq(id.as_str())))
            .execute(conn)
            .map_err(Internal.from_srcf())?;
        Ok(())
    }
}
