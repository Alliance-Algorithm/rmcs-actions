use std::sync::{Arc, LazyLock};

use dashmap::DashMap;
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

pub static CONNECTIONS: LazyLock<Arc<DashMap<String, Arc<Connection>>>> =
    LazyLock::new(|| Arc::new(DashMap::new()));

#[handler]
pub fn websocket_service(
    Path(robot_id): Path<String>,
    ws: WebSocket,
) -> impl IntoResponse {
    // Sync robot id and register it
    log::info!("WebSocket connection established for robot: {}", robot_id);

    ws.on_upgrade(move |socket| async move {
        let (mut sink, mut stream) = socket.split();
        let (ws_writer, mut ws_reader) =
            tokio::sync::mpsc::channel::<message::Message>(100);

        let connection = Arc::new(Connection::new(robot_id, ws_writer));
        CONNECTIONS
            .insert(connection.robot_id.clone(), connection.clone());

        let (shutdown_listener, mut shutdown) =
            tokio::sync::oneshot::channel::<()>();
        
        let connection_c = connection.clone();

        tokio::spawn(async move {
            while let Some(msg) = stream.next().await {
                match msg {
                    Ok(msg) => {
                        if let Message::Text(text) = msg {
                            log::info!("Received WebSocket message: {}", text);
                            if let Err(err) =
                                connection.recv(&text).await
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
                        let msg = Message::Text(serde_json::to_string(&msg).unwrap());
                        log::debug!("Sending WebSocket message: {:?}", msg);
                        if let Err(e) = sink.send(msg).await {
                            log::error!(
                                "Failed to send websocket message: {}",
                                e
                            );
                            break;
                        }
                    }
                    _ = &mut shutdown => {
                        log::info!("Shutting down WebSocket writer");
                        CONNECTIONS.remove(connection_c.robot_id.as_str());
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
