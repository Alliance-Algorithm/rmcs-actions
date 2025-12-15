use futures_util::{FutureExt, future::BoxFuture};
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;
use uuid::Uuid;

use crate::service::{
    action::OnceShot,
    instructions::{InstructionContent, SyncRobotIdMessage},
    message::Message,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncRobotIdRequest {
    pub robot_id: String,
}

pub fn sync_robot_id(
    resp_tx: oneshot::Sender<serde_json::Value>,
    new_robot_id: String,
) -> OnceShot<impl FnOnce(Uuid) -> BoxFuture<'static, anyhow::Result<Message>>>
{
    let _ = resp_tx.send(serde_json::json!({}));
    OnceShot(move |session_id: Uuid| {
        let robot_id = new_robot_id.to_string();
        async move {
            Message::new_instruction_with_uuid(
                session_id,
                InstructionContent::SyncRobotId {
                    message: SyncRobotIdMessage { robot_id },
                },
            )
        }
        .boxed()
    })
}
