use futures_util::FutureExt;
use tokio::sync::oneshot;
use uuid::Uuid;

use crate::service::{
    action::{InitAction, PingPong},
    instructions::{InstructionContent, UpdateBinaryMessage},
    message::Message,
};

pub fn update_binary(
    resp_tx: oneshot::Sender<serde_json::Value>,
    artifact_url: String,
) -> impl InitAction<(), Message> {
    PingPong {
        constructor: move |session_id: Uuid| {
            async move {
                Message::new_instruction_with_uuid(
                    session_id,
                    InstructionContent::UpdateBinary {
                        message: UpdateBinaryMessage { artifact_url },
                    },
                )
            }
            .boxed()
        },
        reader:
            move |_: Uuid, resp_rx: oneshot::Receiver<serde_json::Value>| {
                async move {
                    if let Ok(response) = resp_rx.await {
                        resp_tx.send(response).ok();
                    }
                }
                .boxed()
            },
    }
}
