use std::sync::Arc;

use futures_util::{SinkExt, StreamExt};
use poem::{
    IntoResponse, handler,
    web::{
        Data, Path,
        websocket::{Message, WebSocket},
    },
};

use crate::service::connection::Connection;

pub mod action;
pub mod connection;
pub mod events;
pub mod instructions;
pub mod message;

#[handler]
pub fn websocket_service(
    Path(robot_id): Path<String>,
    ws: WebSocket,
    sender: Data<&tokio::sync::broadcast::Sender<String>>,
) -> impl IntoResponse {
    let sender = sender.clone();
    let mut receiver = sender.subscribe();
    ws.on_upgrade(move |socket| async move {
        let (mut sink, mut stream) = socket.split();

        let connection = Arc::new(Connection::new(robot_id));

        tokio::spawn(async move {
            while let Some(msg) = stream.next().await {
                match msg {
                    Ok(msg) => {
                        if let Message::Text(text) = msg {
                            log::info!("Received WebSocket message: {}", text);
                            if let Err(err) = connection.recv(&text).await {
                                log::error!(
                                    "Failed to process message: {}",
                                    err
                                );
                            }
                        } else if msg.is_ping() || msg.is_pong() {
                            log::debug!("Received WebSocket ping/pong");
                        } else {
                            log::warn!("Unsupported WebSocket message type");
                        }
                    }
                    Err(e) => {
                        log::error!("WebSocket error: {}", e);
                    }
                }
            }
        });

        tokio::spawn(async move {
            loop {
                match receiver.recv().await {
                    Ok(text) => {
                        if let Err(e) = sink.send(Message::Text(text)).await {
                            log::error!("Failed to send websocket message: {}", e);
                            break;
                        }
                    }
                    Err(e) => {
                        log::error!("Broadcast receive error: {}", e);
                        break;
                    }
                }
            }
        })
    })
}
