use crate::AppResult;
use crate::errors::Kind::BadRequest;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct JWT {
    secret_key: String,
}

impl JWT {
    pub fn new(secret_key: String) -> Self {
        Self { secret_key }
    }

    pub fn generate_verification_token<T>(
        &self,
        params: T,
        iat: u64,
        effective_duration: u64,
    ) -> AppResult<String>
    where
        T: Serialize,
    {
        jsonwebtoken::encode(
            &jsonwebtoken::Header::new(Algorithm::HS256),
            &Claims {
                exp: iat + effective_duration,
                other: params,
            },
            &jsonwebtoken::EncodingKey::from_secret(self.secret_key.as_bytes()),
        )
        .map_err(BadRequest.from_srcf())
    }

    pub fn verify_token<T>(&self, token: &str) -> AppResult<T>
    where
        for<'de> T: Deserialize<'de>,
    {
        jsonwebtoken::decode::<Claims<T>>(
            token,
            &DecodingKey::from_secret(self.secret_key.as_bytes()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(BadRequest.from_srcf())
        .map(|v| v.claims.other)
    }
}

#[derive(Serialize, Deserialize)]
struct Claims<T> {
    exp: u64,
    #[serde(flatten)]
    other: T,
}
