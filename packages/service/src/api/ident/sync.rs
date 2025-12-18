use poem_openapi::Object;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Object, Debug, Clone)]
pub struct Sync {
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
    pub mac: String,
    pub name: String,
    pub uuid: String,
}
