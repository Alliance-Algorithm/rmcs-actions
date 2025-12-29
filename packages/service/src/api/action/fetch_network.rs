use poem_openapi::Object;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Object)]
pub struct FetchNetworkRequest {
    pub robot_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Object)]
pub struct FetchNetworkResponse {}
