mod errors;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use async_graphql::async_trait::async_trait;
use aws_sdk_cognitoidentityprovider::types::AttributeType;
use derive_more::Deref;
use jsonwebtoken::{DecodingKey, Validation};
use serde::Deserialize;
use serde_json::Value;

use crate::adapter::AdminAuth;
use crate::domain::admin_user;
use crate::domain::types::email::Email;
use crate::errors::Kind::*;
use crate::AppResult;

#[derive(Clone, Debug)]
pub struct Adapter {
    client: aws_sdk_cognitoidentityprovider::Client,
    jwks: Arc<RwLock<HashMap<String, Jwk>>>,
    user_pool_id: String,
}

impl Adapter {
    pub fn new(client: aws_sdk_cognitoidentityprovider::Client, user_pool_id: String) -> Self {
        Self {
            client,
            jwks: Arc::new(Default::default()),
            user_pool_id,
        }
    }

    async fn refresh_jwks(&self) -> AppResult<()> {
        let jwks = fetch_jwks(self.user_pool_id.as_str()).await?;
        *self.jwks.write().unwrap() = jwks;
        Ok(())
    }
}

#[async_trait]
impl AdminAuth for Adapter {
    async fn verify(&self, token: &str) -> AppResult<admin_user::User> {
        let token_header = jsonwebtoken::decode_header(token).map_err(BadRequest.from_srcf())?;

        let kid = token_header
            .kid
            .ok_or_else(|| BadRequest.with("kid header missing"))?;

        let jwk = self.jwks.read().unwrap().get(&kid).cloned();
        let jwk = match jwk {
            None => {
                self.refresh_jwks().await?;
                let jwk = self.jwks.read().unwrap().get(&kid).cloned();
                match jwk {
                    None => return Err(BadRequest.with("unknown kid")),
                    Some(v) => v,
                }
            }
            Some(v) => v,
        };

        let mut validation = Validation::new(jsonwebtoken::Algorithm::RS256);
        validation.validate_aud = false;

        let claims = jsonwebtoken::decode::<Claims>(
            token,
            &DecodingKey::from_rsa_components(&jwk.n, &jwk.e).map_err(BadRequest.from_srcf())?,
            &validation,
        )
        .map_err(BadRequest.from_srcf())
        .map(|v| v.claims)?;

        let username = claims
            .username()
            .ok_or_else(|| BadRequest.with("username missing"))?;
        let email = claims
            .email()
            .ok_or_else(|| BadRequest.with("email missing"))?;

        Ok(admin_user::User {
            id: username.into(),
            email: email.try_into().map_err(BadRequest.withf())?,
        })
    }

    async fn get(&self, id: &admin_user::Id) -> AppResult<admin_user::User> {
        let response = self
            .client
            .admin_get_user()
            .user_pool_id(self.user_pool_id.clone())
            .username(id.as_str().to_string())
            .send()
            .await?;

        let username = response.username().to_string();
        let attrs = response.user_attributes.unwrap_or_default();
        let mut email: Option<String> = None;
        for attr in attrs {
            if attr.name == "email" {
                email = attr.value;
            }
        }

        Ok(admin_user::User {
            id: username.into(),
            email: email
                .ok_or_else(|| Internal.with("email missing"))?
                .try_into()
                .map_err(Internal.withf())?,
        })
    }

    async fn create(&self, id: admin_user::Id, email: Email) -> AppResult<()> {
        let email_str: String = email.clone().into();
        self.client
            .admin_create_user()
            .user_pool_id(self.user_pool_id.clone())
            .username(id.as_str().to_string())
            .set_user_attributes(Some(
                [("email", email_str), ("email_verified", "true".to_string())]
                    .map(|(k, v)| AttributeType::builder().name(k).value(v).build().unwrap())
                    .to_vec(),
            ))
            .send()
            .await
            .map_err(Internal.from_srcf())?;

        Ok(())
    }

    async fn delete(&self, id: &admin_user::Id) -> AppResult<()> {
        self.client
            .admin_delete_user()
            .user_pool_id(self.user_pool_id.clone())
            .username(id.as_str().to_string())
            .send()
            .await
            .map_err(Internal.from_srcf())?;
        Ok(())
    }
}

async fn fetch_jwks(user_pool_id: &str) -> AppResult<HashMap<String, Jwk>> {
    Ok(reqwest::get(format!(
        "https://cognito-idp.ap-northeast-1.amazonaws.com/{}/.well-known/jwks.json",
        user_pool_id
    ))
    .await
    .map_err(Internal.from_srcf())?
    .json::<KeyResponse>()
    .await
    .map_err(Internal.from_srcf())?
    .keys
    .into_iter()
    .map(|k| (k.kid.clone(), k))
    .collect())
}

#[derive(Clone, Debug, Deserialize)]
struct Jwk {
    e: String,
    // alg: String, RS256
    // kty: String, RSA
    kid: String,
    n: String,
}

#[derive(Debug, Deserialize)]
struct KeyResponse {
    keys: Vec<Jwk>,
}

#[derive(Debug, Clone, Deserialize, Deref)]
#[serde(rename_all = "camelCase")]
struct Claims(serde_json::Map<String, Value>);
impl Claims {
    pub fn username(&self) -> Option<String> {
        self.get_str_val("cognito:username")
    }
    pub fn email(&self) -> Option<String> {
        self.get_str_val("email")
    }

    pub fn get_str_val(&self, key: &str) -> Option<String> {
        self.0.get(key).and_then(|v| match v {
            Value::String(v) => Some(v.to_string()),
            _ => None,
        })
    }
}
