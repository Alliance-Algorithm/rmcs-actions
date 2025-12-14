use std::sync::Arc;

use futures_util::{SinkExt, StreamExt};
use poem::{
    IntoResponse, handler,
    web::{
        Path,
        websocket::{Message, WebSocket},
    },
};
use tokio::select;

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
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| async move {
        let (mut sink, mut stream) = socket.split();

        let connection = Arc::new(Connection::new(robot_id));
        let (ws_writer, mut ws_reader) =
            tokio::sync::mpsc::channel::<message::Message>(100);

        let (shutdown_listener, mut shutdown) =
            tokio::sync::oneshot::channel::<()>();

        tokio::spawn(async move {
            while let Some(msg) = stream.next().await {
                match msg {
                    Ok(msg) => {
                        if let Message::Text(text) = msg {
                            log::info!("Received WebSocket message: {}", text);
                            if let Err(err) =
                                connection.recv(&text, ws_writer.clone()).await
                            {
                                log::error!(
                                    "Failed to process message: {:?}",
                                    err
                                );
                            }
                        } else if msg.is_ping() || msg.is_pong() {
                            log::debug!("Received WebSocket ping/pong");
                        } else if msg.is_close() {
                            log::info!("WebSocket connection closed");
                            let _ = shutdown_listener.send(());
                            break;
                        } else {
                            log::warn!("Unsupported WebSocket message type");
                        }
                    }
                    Err(e) => {
                        log::error!("WebSocket error: {:?}", e);
                    }
                }
            }
        });

        tokio::spawn(async move {
            loop {
                select! {
                    Some(msg) = ws_reader.recv() => {
                        if let Err(e) = sink.send(Message::Text(serde_json::to_string(&msg).unwrap())).await {
                            log::error!(
                                "Failed to send websocket message: {}",
                                e
                            );
                            break;
                        }
                    }
                    _ = &mut shutdown => {
                        log::info!("Shutting down WebSocket writer");
                        break;
                    }
                    else => {
                        log::info!("WebSocket writer channel closed");
                        break;
                    }
                }
            }
        })
    })
}
