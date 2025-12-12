use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use uuid::Uuid;

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub session_id: Uuid,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub local_timestamp: chrono::DateTime<chrono::Utc>,
    pub payload: MessagePayload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum MessagePayload {
    Instruction {
        content: InstructionContent,
    },
    Event {
        detail: serde_json::Value,
    },
    Response {
        content: serde_json::Value,
    },
    Close,
    #[serde(untagged)]
    Unknown(serde_json::Value),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "instruction")]
pub enum InstructionContent {
    #[serde(rename = "sync_robot_id")]
    SyncRobotId {},
    #[serde(untagged)]
    Unknown(serde_json::Value),
}
