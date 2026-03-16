use poem_openapi::Object;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Object)]
pub struct UpdateBinaryRequest {
    pub robot_id: String,
    pub artifact_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Object)]
pub struct UpdateBinaryResponse {
    pub status: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Object)]
pub struct UpdateBinaryAllRequest {
    pub artifact_url: String,
}
