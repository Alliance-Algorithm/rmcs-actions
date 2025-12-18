use poem_openapi::{
    OpenApi,
    payload::{Json, PlainText},
};

use crate::{
    api::{ApiResult, GenericResponse},
    service::{CONNECTIONS, instructions::Instruction},
};

pub mod fetch_network;
pub mod set_robot_name;

pub struct ActionApi;

#[OpenApi]
impl ActionApi {
    #[oai(path = "/action/set_robot_name", method = "post")]
    async fn set_robot_name(
        &self,
        request: Json<set_robot_name::SetRobotNameRequest>,
    ) -> ApiResult<set_robot_name::SetRobotNameResponse> {
        if let Some(conn) = CONNECTIONS.get(&request.robot_name) {
            let _ = conn
                .value()
                .send_instruction(Instruction::SyncRobotId {
                    robot_id: request.new_robot_name.clone(),
                })
                .await;
            Ok(Json(set_robot_name::SetRobotNameResponse))
        } else {
            log::info!(
                "No connection found for robot_id: {}",
                request.robot_name
            );
            Err(GenericResponse::BadRequest(PlainText(
                "robot not connected".to_string(),
            )))
        }
    }

    #[oai(path = "/action/fetch_network", method = "post")]
    async fn fetch_network(
        &self,
        request: Json<fetch_network::FetchNetworkRequest>,
    ) -> ApiResult<serde_json::Value> {
        if let Some(conn) = CONNECTIONS.get(&request.robot_id) {
            let net_info = conn
                .value()
                .send_instruction(Instruction::FetchNetwork {})
                .await;
            match net_info {
                Ok(info) => Ok(Json(info)),
                Err(err) => {
                    log::error!(
                        "Failed to fetch network info from robot {}: {:?}",
                        request.robot_id,
                        err
                    );
                    Err(GenericResponse::BadRequest(PlainText(
                        "failed to fetch network info".to_string(),
                    )))
                }
            }
        } else {
            log::info!(
                "No connection found for robot_id: {}",
                request.robot_id
            );
            Err(GenericResponse::BadRequest(PlainText(
                "robot not connected".to_string(),
            )))
        }
    }
}
