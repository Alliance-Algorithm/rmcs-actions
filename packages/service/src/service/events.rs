use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, oneshot};
use uuid::Uuid;

use crate::service::{action::{Action, InitAction, Streaming}, message::Message};

pub mod heartbeat;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Event {
    Heartbeat,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMessage {
    pub event: Event,
    pub detail: serde_json::Value,
}

pub struct EventSession {
    pub action: Action,
    pub close_listener: oneshot::Sender<()>,
}

pub fn create_event_session(
    event_raw: serde_json::Value,
    session_id: Uuid,
    output_receiver: mpsc::Sender<Message>,
) -> anyhow::Result<EventSession> {
    log::info!("Creating event session for session_id: {}", session_id);
    let event_message: EventMessage = serde_json::from_value(event_raw)?;
    // Channel for streaming outputs from the action: sender goes into the action,
    // receiver is returned for external consumers to read.
    let (action, closer) = match event_message.event {
        Event::Heartbeat => Streaming(heartbeat::heartbeat_task)
            .init_action(session_id, output_receiver),
        Event::Unknown => {
            anyhow::bail!("Unknown event type");
        }
    };
    Ok(EventSession {
        action,
        close_listener: closer,
    })
}
