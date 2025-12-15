use crate::adapter::TaskQueue;
use crate::errors::Kind::*;
use crate::AppResult;
use async_trait::async_trait;
use aws_sdk_sns::Client;

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
impl TaskQueue for Adapter {
    async fn publish(&self, input: serde_json::Value, target: String) -> AppResult<()> {
        let json = serde_json::to_string(&input).map_err(Internal.from_srcf())?;
        self.client
            .publish()
            .topic_arn(&target)
            .message(json)
            .send()
            .await
            .map_err(Internal.from_srcf())?;

        Ok(())
    }
}
