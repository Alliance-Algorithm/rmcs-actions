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

/// Per-robot result within a bulk update operation.
#[derive(Debug, Clone, Serialize, Deserialize, Object)]
pub struct RobotUpdateResult {
    pub robot_id: String,
    pub status: String,
    pub message: String,
}

/// Response for the update_binary_all endpoint, aggregating
/// per-robot results.
#[derive(Debug, Clone, Serialize, Deserialize, Object)]
pub struct UpdateBinaryAllResponse {
    pub status: String,
    pub results: Vec<RobotUpdateResult>,
}
