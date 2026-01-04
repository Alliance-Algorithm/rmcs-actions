use serde::{Deserialize, Serialize, de::DeserializeOwned};
use tokio::sync::{mpsc, oneshot};
use uuid::Uuid;

use crate::service::{
    action::{Action, InitAction},
    message::Message,
};

pub mod fetch_network;
pub mod sync_robot_name;
pub mod update_metadata;

pub struct InstructionSession {
    pub action: Action,
    pub close_listener: oneshot::Sender<()>,
}

pub fn create_instruction_session<I>(
    session_id: Uuid,
    output_receiver: mpsc::Sender<Message>,
    instruction: impl InitAction<I, Message> + 'static,
    on_complete: impl FnOnce() + Send + 'static,
) -> InstructionSession
where
    I: 'static + Send + Sync + DeserializeOwned,
{
    log::info!(
        "Creating instruction session for session_id: {}",
        session_id
    );
    let (action, closer) =
        instruction.init_action(session_id, output_receiver, on_complete);
    InstructionSession {
        action,
        close_listener: closer,
    }
}

pub enum Instruction {
    SyncRobotName { robot_name: String },
    FetchNetwork {},
    UpdateMetadata {},
}

impl Instruction {
    pub fn into_session_compatible<F: FnOnce() + Send + 'static>(
        self,
        session_id: Uuid,
        resp_tx: oneshot::Sender<serde_json::Value>,
    ) -> impl FnOnce(mpsc::Sender<Message>, F) -> InstructionSession {
        move |output_receiver: mpsc::Sender<Message>, on_complete: F| match self
        {
            Instruction::SyncRobotName { robot_name } => {
                create_instruction_session::<
                    sync_robot_name::SyncRobotNameRequest,
                >(
                    session_id,
                    output_receiver,
                    sync_robot_name::sync_robot_name(resp_tx, robot_name),
                    on_complete,
                )
            }
            Instruction::FetchNetwork {} => create_instruction_session::<()>(
                session_id,
                output_receiver,
                fetch_network::fetch_network(resp_tx),
                on_complete,
            ),
            Instruction::UpdateMetadata {} => create_instruction_session::<()>(
                session_id,
                output_receiver,
                update_metadata::update_metadata(resp_tx),
                on_complete,
            ),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "instruction")]
pub enum InstructionContent {
    #[serde(rename = "sync_robot_name")]
    SyncRobotName { message: SyncRobotNameMessage },
    #[serde(rename = "fetch_network")]
    FetchNetwork {},
    #[serde(rename = "update_metadata")]
    UpdateMetadata {},
    #[serde(untagged)]
    Unknown(serde_json::Value),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncRobotNameMessage {
    pub robot_name: String,
}
