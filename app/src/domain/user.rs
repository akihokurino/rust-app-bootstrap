use crate::adapter::DbConn;
use crate::domain::types::string::impl_len_restricted_string_model;
use crate::domain::types::time::{now, LocalDateTime};
use crate::domain::HasId;
use crate::AppResult;
use async_trait::async_trait;

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

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find(&self, db: DbConn<'_>) -> AppResult<Vec<User>>;
    async fn get(&self, db: DbConn<'_>, id: &Id) -> AppResult<User>;
    async fn get_multi(&self, db: DbConn<'_>, ids: Vec<&Id>) -> AppResult<Vec<User>>;
    async fn insert(&self, db: DbConn<'_>, user: User) -> AppResult<()>;
    async fn update(&self, db: DbConn<'_>, user: User) -> AppResult<()>;
    async fn delete(&self, db: DbConn<'_>, id: &Id) -> AppResult<()>;
}
