use poem_openapi::Object;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Object, Debug, Clone)]
pub struct Sync {
    pub robot_id: String,
    pub mac: String,
    pub name: String,
    pub uuid: String,
}

#[derive(Serialize, Deserialize, Object, Debug, Clone)]
pub struct SyncResponse {
    pub success: bool,
}

#[derive(Serialize, Deserialize, Object, Debug, Clone)]
pub struct RetrieveResponse {
    pub robot_id: String,
    pub mac: String,
    pub name: String,
    pub uuid: String,
}
