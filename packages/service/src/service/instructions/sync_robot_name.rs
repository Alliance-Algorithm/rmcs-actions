use futures_util::{FutureExt, future::BoxFuture};
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;
use uuid::Uuid;

use crate::service::{
    action::OnceShot,
    instructions::{InstructionContent, SyncRobotNameMessage},
    message::Message,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncRobotNameRequest {
    pub robot_name: String,
}

pub fn sync_robot_name(
    resp_tx: oneshot::Sender<serde_json::Value>,
    new_robot_name: String,
) -> OnceShot<impl FnOnce(Uuid) -> BoxFuture<'static, anyhow::Result<Message>>>
{
    let _ = resp_tx.send(serde_json::json!({}));
    OnceShot(move |session_id: Uuid| {
        let robot_name = new_robot_name.to_string();
        async move {
            Message::new_instruction_with_uuid(
                session_id,
                InstructionContent::SyncRobotName {
                    message: SyncRobotNameMessage { robot_name },
                },
            )
        }
        .boxed()
    })
}
