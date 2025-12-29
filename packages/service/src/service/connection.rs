use std::sync::Arc;

use dashmap::DashMap;
use serde::de::DeserializeOwned;
use tokio::sync::{mpsc, oneshot};
use uuid::Uuid;

use crate::service::{
    action::Action,
    events,
    instructions::Instruction,
    message::{Message, MessagePayload},
};

pub struct Connection {
    pub sessions: Arc<DashMap<Uuid, (Action, oneshot::Sender<()>)>>,
    pub robot_id: String,
    pub writer: mpsc::Sender<Message>,
}

impl Connection {
    pub fn new(robot_id: String, writer: mpsc::Sender<Message>) -> Self {
        Connection {
            sessions: Arc::new(DashMap::new()),
            robot_id,
            writer,
        }
    }

    pub async fn recv(&self, msg: &str) -> anyhow::Result<()> {
        let message: Message = serde_json::from_str(msg)?;
        let session_id = message.session_id;
        let payload = message.payload;
        self.process_session(session_id, payload).await
    }

    pub async fn send_instruction<T: DeserializeOwned>(
        &self,
        instruction: Instruction,
    ) -> anyhow::Result<T> {
        let (resp_tx, resp_rx) = oneshot::channel();
        let sessions = self.sessions.clone();
        let session_id = Uuid::new_v4();
        let session = instruction.into_session_compatible(session_id, resp_tx)(
            self.writer.clone(),
            move || {
                sessions.remove(&session_id);
            },
        );
        self.sessions
            .insert(session_id, (session.action, session.close_listener));
        let response = resp_rx.await?;
        Ok(serde_json::from_value(response)?)
    }

    async fn process_session(
        &self,
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
                let sessions = self.sessions.clone();
                let event_session = events::create_event_session(
                    content,
                    session_id,
                    self.writer.clone(),
                    move || {
                        sessions.remove(&session_id);
                    },
                )?;
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
