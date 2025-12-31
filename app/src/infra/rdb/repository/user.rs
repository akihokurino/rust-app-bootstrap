use crate::adapter::DbConn;
use crate::domain::user::{Id, User, UserRepository};
use crate::infra::rdb::repository;
use crate::infra::rdb::types::prelude::*;
use crate::infra::rdb::types::users;
use crate::AppResult;
use async_trait::async_trait;

impl TryFrom<users::Model> for User {
    type Error = String;
    fn try_from(v: users::Model) -> Result<Self, Self::Error> {
        Ok(Self {
            id: v.id.into(),
            name: v.name.try_into()?,
            birthdate: v.birthdate.into(),
            gender: v.gender.try_into()?,
            created_at: v.created_at.into(),
            updated_at: v.updated_at.into(),
        })
    }
}

impl From<User> for users::Model {
    fn from(v: User) -> Self {
        Self {
            id: v.id.into(),
            name: v.name.into(),
            birthdate: v.birthdate.into(),
            gender: v.gender.into(),
            created_at: v.created_at.into(),
            updated_at: v.updated_at.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Repository;

impl Repository {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl UserRepository for Repository {
    async fn find(&self, db: DbConn<'_>) -> AppResult<Vec<User>> {
        repository::find::<Users, User, _>(db, users::Column::CreatedAt).await
    }

    async fn get(&self, db: DbConn<'_>, id: &Id) -> AppResult<User> {
        repository::get::<Users, User>(db, id).await
    }

    async fn get_multi(&self, db: DbConn<'_>, ids: Vec<&Id>) -> AppResult<Vec<User>> {
        repository::get_multi::<Users, User, _>(db, users::Column::Id, ids).await
    }

    async fn insert(&self, db: DbConn<'_>, user: User) -> AppResult<()> {
        repository::insert::<Users, User>(db, user).await
    }

    async fn update(&self, db: DbConn<'_>, user: User) -> AppResult<()> {
        repository::update::<Users, User, _>(db, users::Column::Id, user).await
    }

    async fn delete(&self, db: DbConn<'_>, id: &Id) -> AppResult<()> {
        repository::delete::<Users>(db, id).await
    }
}
