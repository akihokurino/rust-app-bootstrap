use crate::adapter::TaskQueue;
use crate::errors::Kind::*;
use crate::AppResult;
use async_trait::async_trait;
use aws_sdk_sqs::Client;

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
            .send_message()
            .queue_url(&target)
            .message_body(json)
            .send()
            .await
            .map_err(Internal.from_srcf())?;

        Ok(())
    }
}
