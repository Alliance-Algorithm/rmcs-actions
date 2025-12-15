use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use uuid::Uuid;

use crate::service::instructions::InstructionContent;

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub session_id: Uuid,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub local_timestamp: chrono::DateTime<chrono::Utc>,
    pub payload: MessagePayload,
}

#[allow(dead_code)]
impl Message {
    pub fn new_event_with_uuid(
        session_id: Uuid,
        payload: impl Serialize,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            session_id,
            local_timestamp: chrono::Utc::now(),
            payload: MessagePayload::Event {
                content: serde_json::to_value(payload)?,
            },
        })
    }

    pub fn new_response_with_uuid(
        session_id: Uuid,
        payload: impl Serialize,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            session_id,
            local_timestamp: chrono::Utc::now(),
            payload: MessagePayload::Response {
                content: serde_json::to_value(payload)?,
            },
        })
    }

    pub fn new_instruction_with_uuid(
        session_id: Uuid,
        content: InstructionContent,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            session_id,
            local_timestamp: chrono::Utc::now(),
            payload: MessagePayload::Instruction { content },
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum MessagePayload {
    Instruction {
        content: InstructionContent,
    },
    Event {
        content: serde_json::Value,
    },
    Response {
        content: serde_json::Value,
    },
    Close,
    #[serde(untagged)]
    Unknown(serde_json::Value),
}
