use crate::adapter::DbConn;
use crate::domain::types::string::impl_len_restricted_string_model;
use crate::domain::types::time::{now, Date, LocalDateTime};
use crate::domain::HasId;
use crate::AppResult;
use async_trait::async_trait;
use strum::IntoEnumIterator;

pub type Id = crate::domain::Id<User>;
#[derive(Debug, Clone)]
pub struct User {
    pub id: Id,
    pub name: Name,
    pub birthdate: Date,
    pub gender: Gender,
    pub created_at: LocalDateTime,
    pub updated_at: LocalDateTime,
}
impl User {
    pub fn new(id: Id, name: Name, birthdate: Date, gender: Gender) -> Self {
        Self {
            id,
            name,
            birthdate,
            gender,
            created_at: now(),
            updated_at: now(),
        }
    }

    pub fn update(self, name: Name, birthdate: Date, gender: Gender) -> Self {
        Self {
            name,
            birthdate,
            gender,
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

#[derive(
    Debug,
    Clone,
    Copy,
    Eq,
    PartialEq,
    strum_macros::EnumString,
    strum_macros::Display,
    strum_macros::EnumIter,
    async_graphql::Enum,
)]
pub enum Gender {
    #[strum(to_string = "男性", serialize = "Male")]
    Male,
    #[strum(to_string = "女性", serialize = "Female")]
    Female,
}
impl Gender {
    pub fn all() -> Vec<Self> {
        Self::iter().collect()
    }
}
impl TryFrom<String> for Gender {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse().map_err(|e| format!("error: {:?}", e))
    }
}
impl Into<String> for Gender {
    fn into(self) -> String {
        format!("{:?}", self)
    }
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find(&self, db: DbConn<'_>) -> AppResult<Vec<User>>;
    async fn get(&self, db: DbConn<'_>, id: &Id) -> AppResult<User>;
    async fn get_multi(&self, db: DbConn<'_>, ids: Vec<&Id>) -> AppResult<Vec<User>>;
    async fn insert(&self, db: DbConn<'_>, user: User) -> AppResult<()>;
    async fn update(&self, db: DbConn<'_>, user: User) -> AppResult<()>;
    async fn delete(&self, db: DbConn<'_>, id: &Id) -> AppResult<()>;
}
