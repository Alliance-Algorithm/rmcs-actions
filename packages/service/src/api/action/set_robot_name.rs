use poem_openapi::Object;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Object)]
pub struct SetRobotNameRequest {
    pub robot_name: String,
    pub new_robot_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Object)]
pub struct SetRobotNameResponse;
