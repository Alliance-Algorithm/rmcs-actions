use poem_openapi::Object;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Object)]
pub struct SetRobotIdRequest {
    pub robot_id: String,
    pub new_robot_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Object)]
pub struct SetRobotIdResponse {
    pub success: bool,
}
