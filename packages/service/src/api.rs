pub mod action;
pub mod ident;
pub mod meta;

use poem_openapi::{
    OpenApi,
    payload::{Json, PlainText},
};

use crate::service::{CONNECTIONS, instructions::Instruction};

pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/ping", method = "get")]
    async fn pong(&self) -> PlainText<&'static str> {
        PlainText("pong")
    }

    #[oai(path = "/meta/version", method = "get")]
    async fn version(&self) -> Json<meta::version::Version> {
        Json(meta::version::Version::default())
    }

    #[oai(path = "/action/set_robot_id", method = "post")]
    async fn set_robot_id(
        &self,
        request: Json<action::set_robot_id::SetRobotIdRequest>,
    ) -> Json<action::set_robot_id::SetRobotIdResponse> {
        if let Some(conn) = CONNECTIONS.get(&request.robot_id) {
            let _ = conn
                .value()
                .send_instruction(Instruction::SyncRobotId {
                    robot_id: request.new_robot_id.clone(),
                })
                .await;
            Json(action::set_robot_id::SetRobotIdResponse { success: true })
        } else {
            log::info!(
                "No connection found for robot_id: {}",
                request.robot_id
            );
            Json(action::set_robot_id::SetRobotIdResponse { success: false })
        }
    }

    #[oai(path = "/action/fetch_network", method = "post")]
    async fn fetch_network(
        &self,
        request: Json<action::fetch_network::FetchNetworkRequest>,
    ) -> Json<serde_json::Value> {
        if let Some(conn) = CONNECTIONS.get(&request.robot_id) {
            let net_info = conn
                .value()
                .send_instruction(Instruction::FetchNetwork {})
                .await;
            match net_info {
                Ok(info) => Json(info),
                Err(err) => {
                    log::error!(
                        "Failed to fetch network info from robot {}: {:?}",
                        request.robot_id,
                        err
                    );
                    Json(
                        serde_json::json!({"error": "failed to fetch network info"}),
                    )
                }
            }
        } else {
            log::info!(
                "No connection found for robot_id: {}",
                request.robot_id
            );
            Json(serde_json::json!({"error": "robot not connected"}))
        }
    }
}
