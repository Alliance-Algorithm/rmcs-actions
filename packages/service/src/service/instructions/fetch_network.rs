use futures_util::{FutureExt, future::BoxFuture};
use tokio::sync::oneshot;
use uuid::Uuid;

use crate::service::{action::PingPong, message::Message};

pub fn fetch_network(
    resp_tx: oneshot::Sender<serde_json::Value>,
) -> PingPong<
    impl FnOnce(Uuid) -> BoxFuture<'static, anyhow::Result<Message>>,
    impl FnOnce(
        Uuid,
        oneshot::Receiver<serde_json::Value>,
    ) -> BoxFuture<'static, ()>,
> {
    PingPong {
        constructor: move |session_id: Uuid| {
            async move {
                Message::new_instruction_with_uuid(
                    session_id,
                    crate::service::instructions::InstructionContent::FetchNetwork {},
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
