use serde::{Deserialize, Serialize};

// SNS event types for Lambda
#[derive(Serialize, Deserialize)]
pub struct SnsEventData {
    #[serde(rename = "Records")]
    pub records: Vec<SnsRecord>,
}
#[derive(Serialize, Deserialize)]
pub struct SnsRecord {
    #[serde(rename = "Sns")]
    pub sns: SnsMessage,
}
#[derive(Serialize, Deserialize)]
pub struct SnsMessage {
    #[serde(rename = "Message")]
    pub message: String,
}

// SQS event types for Lambda
#[derive(Serialize, Deserialize)]
pub struct SqsEventData {
    #[serde(rename = "Records")]
    pub records: Vec<SqsRecord>,
}
#[derive(Serialize, Deserialize)]
pub struct SqsRecord {
    pub body: String,
}

// Async task types
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AsyncTaskPayload {
    pub name: String,
}

// Sync task types
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SyncTaskPayload {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SyncTaskResponse {
    pub name: String,
}
