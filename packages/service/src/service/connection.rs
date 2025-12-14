use dashmap::DashMap;
use tokio::sync::{mpsc, oneshot};
use uuid::Uuid;

use crate::service::{
    action::Action,
    events,
    message::{Message, MessagePayload},
};

pub struct Connection {
    pub sessions: DashMap<Uuid, (Action, oneshot::Sender<()>)>,
    #[allow(unused)]
    pub robot_id: String,
}

impl Connection {
    pub fn new(robot_id: String) -> Self {
        Connection {
            sessions: DashMap::new(),
            robot_id,
        }
    }

    pub async fn recv(&self, msg: &str, ws_sender: mpsc::Sender<Message>) -> anyhow::Result<()> {
        let message: Message = serde_json::from_str(msg)?;
        let session_id = message.session_id;
        let payload = message.payload;
        self.process_session(ws_sender, session_id, payload).await
    }

    async fn process_session(
        &self,
        ws_sender: mpsc::Sender<Message>,
        session_id: Uuid,
        message_payload: MessagePayload,
    ) -> anyhow::Result<()> {
        match message_payload {
            MessagePayload::Instruction { .. } => {
                log::error!(
                    "Invalid message payload: Instructions shall be sent by the server."
                );
            }
            MessagePayload::Event { content } => {
                log::info!("Processing event for session_id: {}", session_id);
                let event_session =
                    events::create_event_session(content, session_id, ws_sender)?;
                self.sessions.insert(
                    session_id,
                    (event_session.action, event_session.close_listener),
                );
            }
            MessagePayload::Response { content } => {
                if let Some(action) = self.sessions.get_mut(&session_id) {
                    let _ = action.0.resume(content).await?;
                } else {
                    log::error!(
                        "Received unknown session response for {session_id}"
                    )
                }
            }
            MessagePayload::Close => {
                if let Some(handle) = self.sessions.remove(&session_id) {
                    let (_, (action, close_sender)) = handle;
                    // Abort the running task and notify close
                    let _ = close_sender.send(());
                    action.abort();
                } else {
                    log::warn!(
                        "Received close for unknown session {session_id}"
                    );
                }
                return Ok(());
            }
            _ => {}
        }

        Ok(())
    }
}
