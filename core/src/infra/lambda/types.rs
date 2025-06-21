use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub error_message: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SyncTaskPayload {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SyncTaskResponse {
    pub name: String,
}
