use serde::{Deserialize, Serialize};

// SNS event types for Lambda
#[derive(Serialize, Deserialize)]
pub struct EventData {
    #[serde(rename = "Records")]
    pub records: Vec<Record>,
}

#[derive(Serialize, Deserialize)]
pub struct Record {
    #[serde(rename = "Sns")]
    pub sns: Sns,
}

#[derive(Serialize, Deserialize)]
pub struct Sns {
    #[serde(rename = "Message")]
    pub message: String,
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
