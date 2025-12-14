use crate::adapter::{UserAuth, UserPrincipal};
use crate::errors::Kind::BadRequest;
use crate::{domain, AppResult};
use async_graphql::async_trait::async_trait;

#[derive(Clone)]
pub struct MockAdapter;

impl MockAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for MockAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl UserAuth for MockAdapter {
    async fn verify(&self, token: &str) -> AppResult<domain::user::Id> {
        if token.is_empty() {
            return Err(BadRequest.with("token is empty"));
        }
        Ok(token.to_string().into())
    }

    async fn get(&self, id: &domain::user::Id) -> AppResult<UserPrincipal> {
        Ok(UserPrincipal {
            uid: Some(id.as_str().to_string()),
            email: None,
            provider_ids: vec![],
            last_login_at: None,
        })
    }

    async fn delete(&self, _id: &domain::user::Id) -> AppResult<()> {
        Ok(())
    }
}
