use crate::adapter::UserAuth;
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
    async fn delete(&self, _user_id: &domain::user::Id) -> AppResult<()> {
        Ok(())
    }

    async fn verify(&self, token: &str) -> AppResult<domain::user::Id> {
        if token.is_empty() {
            return Err(BadRequest.with("token is empty"));
        }
        Ok(token.to_string().into())
    }
}
