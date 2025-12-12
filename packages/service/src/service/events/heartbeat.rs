use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, oneshot};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct HeartbeatDetail {}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct HeartbeatResponse {}

pub async fn heartbeat_task(
    mut receiver: mpsc::Receiver<serde_json::Value>,
    sender: mpsc::Sender<serde_json::Value>,
    mut close_listener: oneshot::Receiver<()>,
) -> anyhow::Result<()> {
    loop {
        tokio::select! {
            Some(detail) = receiver.recv() => {
                let _ = serde_json::from_value::<HeartbeatDetail>(detail)?;
                // Process heartbeat detail if needed
                let response = HeartbeatResponse {};
                tokio::time::sleep(std::time::Duration::from_millis(HEARTBEAT_ELAPSE_MS)).await;
                sender.send(serde_json::to_value(response)?).await?;
            }
            _ = &mut close_listener => {
                log::info!("Heartbeat task received close signal.");
                break;
            }
        }
    }
    Ok(())
}

const HEARTBEAT_ELAPSE_MS: u64 = 5000;
