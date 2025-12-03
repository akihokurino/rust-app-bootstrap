pub mod types;

use crate::adapter::RemoteFunction;
use crate::errors::Kind::Internal;
use crate::infra::lambda::types::ErrorResponse;
use crate::AppResult;
use async_trait::async_trait;
use aws_sdk_lambda::primitives::Blob;
use aws_sdk_lambda::Client;

#[derive(Clone, Debug)]
pub struct Adapter {
    client: Client,
}

impl Adapter {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl RemoteFunction for Adapter {
    async fn invoke(&self, input: serde_json::Value, arn: String) -> AppResult<serde_json::Value> {
        let json = serde_json::to_string(&input).map_err(Internal.from_srcf())?;
        let resp = self
            .client
            .invoke()
            .function_name(arn)
            .payload(Blob::new(json.into_bytes()))
            .send()
            .await
            .map_err(Internal.from_srcf())?;
        let payload = resp.payload.unwrap();
        let payload = String::from_utf8(payload.into_inner()).ok().unwrap();

        if let Some(error) = serde_json::from_str::<ErrorResponse>(&payload).ok() {
            return Err(Internal.with(error.error_message));
        }
        let output = serde_json::from_str(&payload).map_err(Internal.from_srcf())?;

        Ok(output)
    }
}
