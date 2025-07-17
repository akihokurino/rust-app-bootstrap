use crate::domain::user::{Id, User};
use crate::errors::Kind::Internal;
use crate::{domain, AppResult};
use chrono::{DateTime, Utc};
use sqlx::{FromRow, Postgres};

#[derive(FromRow)]
struct UserModel {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
impl TryFrom<UserModel> for User {
    type Error = String;
    fn try_from(v: UserModel) -> Result<Self, Self::Error> {
        Ok(Self {
            id: domain::user::Id::from(v.id),
            name: v.name.try_into()?,
            created_at: v.created_at.into(),
            updated_at: v.updated_at.into(),
        })
    }
}
impl From<User> for UserModel {
    fn from(user: User) -> Self {
        UserModel {
            id: user.id.into(),
            name: user.name.into(),
            created_at: user.created_at.into(),
            updated_at: user.updated_at.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserRepository {}
impl UserRepository {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn find<'a, E>(&self, pool: E) -> AppResult<Vec<User>>
    where
        E: sqlx::Executor<'a, Database = Postgres>,
    {
        sqlx::query_as!(UserModel, "SELECT * FROM users")
            .fetch_all(pool)
            .await
            .map_err(Internal.from_srcf())?
            .into_iter()
            .map(|v| v.try_into().map_err(|v| Internal.with(v)))
            .collect()
    }

    pub async fn get<'a, E>(&self, pool: E, id: &Id) -> AppResult<User>
    where
        E: sqlx::Executor<'a, Database = Postgres>,
    {
        sqlx::query_as!(UserModel, "SELECT * FROM users WHERE id = $1", id.as_str())
            .fetch_one(pool)
            .await
            .map_err(Internal.from_srcf())?
            .try_into()
            .map_err(Internal.withf())
    }

    pub async fn get_multi<'a, E>(&self, pool: E, ids: Vec<&Id>) -> AppResult<Vec<User>>
    where
        E: sqlx::Executor<'a, Database = Postgres>,
    {
        let ids: Vec<String> = ids.iter().map(|id| id.as_str().to_string()).collect();

        sqlx::query_as!(UserModel, "SELECT * FROM users WHERE id = ANY($1)", &ids)
            .fetch_all(pool)
            .await
            .map_err(Internal.from_srcf())?
            .into_iter()
            .map(|v| v.try_into().map_err(Internal.withf()))
            .collect()
    }

    pub async fn insert<'a, E>(&self, pool: E, entity: User) -> AppResult<()>
    where
        E: sqlx::Executor<'a, Database = Postgres>,
    {
        let model: UserModel = entity.into();

        sqlx::query!(
            "
INSERT INTO users
    (id, name, created_at, updated_at)
VALUES
    ($1, $2, $3, $4)",
            model.id,
            model.name,
            model.created_at,
            model.updated_at
        )
        .execute(pool)
        .await
        .map_err(Internal.from_srcf())?;

        Ok(())
    }

    pub async fn update<'a, E>(&self, pool: E, entity: User) -> AppResult<()>
    where
        E: sqlx::Executor<'a, Database = Postgres>,
    {
        let model: UserModel = entity.into();

        sqlx::query!(
            "
UPDATE users
SET
    name = $2,
    created_at = $3,
    updated_at = $4
WHERE id = $1",
            model.id,
            model.name,
            model.created_at,
            model.updated_at
        )
        .execute(pool)
        .await
        .map_err(Internal.from_srcf())?;

        Ok(())
    }

    pub async fn delete<'a, E>(&self, pool: E, id: &Id) -> AppResult<()>
    where
        E: sqlx::Executor<'a, Database = Postgres>,
    {
        sqlx::query!("DELETE FROM users WHERE id = $1", id.as_str())
            .execute(pool)
            .await
            .map_err(Internal.from_srcf())?;

        Ok(())
    }
}
