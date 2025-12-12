use poem_openapi::Object;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Object, Debug, Clone)]
pub struct WhoAmI {
    pub username: String,
    pub mac: String,
}

#[derive(Serialize, Deserialize, Object, Debug, Clone)]
pub struct WhoAmIResponse {
    pub robot_id: String,
}
