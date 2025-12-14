use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, oneshot};

use crate::service::message::Message;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct HeartbeatDetail {}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct HeartbeatResponse {}

pub async fn heartbeat_task(
    session_id: uuid::Uuid,
    mut receiver: mpsc::Receiver<serde_json::Value>,
    sender: mpsc::Sender<Message>,
    mut close_listener: oneshot::Receiver<()>,
) -> anyhow::Result<()> {
    loop {
        tokio::select! {
            Some(detail) = receiver.recv() => {
                let _ = serde_json::from_value::<HeartbeatDetail>(detail)?;
                // Process heartbeat detail if needed
                let response = HeartbeatResponse {};
                sender.send(Message::new_response_with_uuid(session_id, response)?).await?;
                log::debug!("Heartbeat response sent.");
            }
            _ = &mut close_listener => {
                log::info!("Heartbeat task received close signal.");
                break;
            }
        }
    }
    Ok(())
}
