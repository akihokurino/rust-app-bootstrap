use crate::adapter::ImageCdn;
use crate::domain::types::asset_key::AssetKey;
use crate::domain::types::image_size::ImageSize;
use crate::errors::Kind::*;
use crate::AppResult;
use async_graphql::async_trait::async_trait;
use base64::prelude::*;
use cloudfront_sign::{get_signed_url, SignedOptions};
use http::Uri;
use std::borrow::Cow;
use std::time::Duration;

#[derive(Clone)]
pub struct Adapter {
    domain: String,
    key_pair_id: String,
    private_key: String,
}

impl Adapter {
    pub fn new(domain: String, key_pair_id: String, private_key_base64: String) -> AppResult<Self> {
        let private_key_bytes = BASE64_STANDARD
            .decode(&private_key_base64)
            .map_err(Internal.from_srcf())?;
        let private_key = String::from_utf8(private_key_bytes).map_err(Internal.from_srcf())?;

        Ok(Self {
            domain,
            key_pair_id,
            private_key,
        })
    }
}

#[async_trait]
impl ImageCdn for Adapter {
    async fn presign_for_get(&self, key: &AssetKey, size: ImageSize) -> AppResult<Uri> {
        let expires_in = Duration::from_secs(60 * 60);
        let expires_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + expires_in.as_secs();

        let base_url = format!("https://{}/{}", self.domain, key.to_string());
        let url_with_size = format!("{}?{}", base_url, size.query_param());

        let options = SignedOptions {
            key_pair_id: Cow::Owned(self.key_pair_id.clone()),
            private_key: Cow::Owned(self.private_key.clone()),
            date_less_than: expires_at,
            ..Default::default()
        };

        let signed_url = get_signed_url(&url_with_size, &options).map_err(Internal.from_srcf())?;

        signed_url.parse().map_err(Internal.from_srcf())
    }
}
