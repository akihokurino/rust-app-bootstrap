use crate::adapter::{UserAuth, UserPrincipal};
use crate::errors::Kind::{BadRequest, Internal, NotFound};
use crate::{domain, AppResult};
use async_graphql::async_trait::async_trait;
use google_identitytoolkit3::api::{
    IdentitytoolkitRelyingpartyDeleteAccountRequest,
    IdentitytoolkitRelyingpartyGetAccountInfoRequest,
};
use google_identitytoolkit3::{hyper, hyper_rustls};
use jsonwebtoken::{DecodingKey, Validation};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub type IdentityToolkit = google_identitytoolkit3::IdentityToolkit<
    hyper_rustls::HttpsConnector<hyper::client::HttpConnector>,
>;

#[derive(Clone)]
pub struct Adapter {
    identity_toolkit: IdentityToolkit,
    jwks: Arc<RwLock<HashMap<String, Jwk>>>,
    project_id: String,
}
impl Adapter {
    pub async fn new(project_id: String, identity_toolkit: IdentityToolkit) -> Self {
        Self {
            identity_toolkit,
            jwks: Arc::new(RwLock::new(
                fetch_jwks()
                    .await
                    .expect("failed to init firebase auth adapter"),
            )),
            project_id,
        }
    }

    async fn refresh_jwks(&self) -> AppResult<()> {
        let jwks = fetch_jwks().await?;
        *self.jwks.write().unwrap() = jwks;
        Ok(())
    }
}

#[async_trait]
impl UserAuth for Adapter {
    async fn verify(&self, token: &str) -> AppResult<domain::user::Id> {
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
        validation.set_audience(&[&self.project_id]);

        let token = jsonwebtoken::decode::<Claims>(
            token,
            &DecodingKey::from_rsa_components(&jwk.n, &jwk.e).map_err(BadRequest.from_srcf())?,
            &validation,
        )
        .map_err(BadRequest.from_srcf())?;

        Ok(token.claims.sub.into())
    }

    async fn get(&self, id: &domain::user::Id) -> AppResult<UserPrincipal> {
        let mut request = IdentitytoolkitRelyingpartyGetAccountInfoRequest::default();
        request.local_id = Some(vec![id.as_str().to_string()]);

        let result = self
            .identity_toolkit
            .relyingparty()
            .get_account_info(request)
            .doit()
            .await
            .map_err(Internal.from_srcf())?;
        if let Some(users) = result.1.users {
            if users.is_empty() {
                return Err(NotFound.with("user not found"));
            }
            let user = users.first().unwrap();
            Ok(UserPrincipal {
                uid: Some(user.local_id.clone().unwrap()),
                email: user.clone().email.map(|v| v.clone()),
                provider_ids: user
                    .provider_user_info
                    .clone()
                    .map(|v| {
                        v.into_iter()
                            .filter_map(|v| v.provider_id)
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default(),
                last_login_at: user.last_login_at.clone(),
            })
        } else {
            Err(NotFound.with("user not found"))
        }
    }

    async fn delete(&self, id: &domain::user::Id) -> AppResult<()> {
        self.identity_toolkit
            .relyingparty()
            .delete_account(IdentitytoolkitRelyingpartyDeleteAccountRequest {
                local_id: Some(id.as_str().to_string()),
                ..Default::default()
            })
            .doit()
            .await
            .map_err(|v| match v {
                google_identitytoolkit3::Error::BadRequest(_) => NotFound.default(),
                _ => Internal.from_src(v),
            })?;
        Ok(())
    }
}

async fn fetch_jwks() -> AppResult<HashMap<String, Jwk>> {
    Ok(reqwest::get(
        "https://www.googleapis.com/service_accounts/v1/jwk/securetoken@system.gserviceaccount.com",
    )
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
    kid: String,
    n: String,
}

#[derive(Debug, Deserialize)]
struct KeyResponse {
    keys: Vec<Jwk>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Claims {
    sub: String,
}
