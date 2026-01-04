use futures_util::{FutureExt, future::BoxFuture};
use tokio::sync::oneshot;
use uuid::Uuid;

use crate::service::{
    action::OnceShot, instructions::InstructionContent, message::Message,
};

pub fn update_metadata(
    resp_tx: oneshot::Sender<serde_json::Value>,
) -> OnceShot<impl FnOnce(Uuid) -> BoxFuture<'static, anyhow::Result<Message>>>
{
    let _ = resp_tx.send(serde_json::json!({}));
    OnceShot(move |session_id| {
        async move {
            Message::new_instruction_with_uuid(
                session_id,
                InstructionContent::UpdateMetadata {},
            )
        }
        .boxed()
    })
}
