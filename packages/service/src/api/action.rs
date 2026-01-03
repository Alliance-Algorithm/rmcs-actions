use poem_openapi::{
    OpenApi,
    payload::{Json, PlainText},
};

use crate::{
    api::{AnyDeserialize, ApiResult, GenericResponse},
    database::with_database,
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
        if let Some(conn) = CONNECTIONS.get(&request.robot_uuid) {
            let _ = conn
                .value()
                .send_instruction::<AnyDeserialize>(
                    Instruction::SyncRobotName {
                        robot_name: request.new_robot_name.clone(),
                    },
                )
                .await?;
            let _ = with_database(|db| {
                db.set_robot_name(&request.robot_uuid, &request.new_robot_name)
            })?
            .await?;
            Ok(Json(set_robot_name::SetRobotNameResponse))
        } else {
            log::info!(
                "No connection found for robot_id: {}",
                request.robot_uuid
            );
            Err(GenericResponse::BadRequest(PlainText(
                "robot not connected".to_string(),
            )))
        }
    }

    #[oai(path = "/action/refresh_network", method = "post")]
    async fn refresh_network(
        &self,
        request: Json<fetch_network::FetchNetworkRequest>,
    ) -> ApiResult<fetch_network::FetchNetworkResponse> {
        if let Some(conn) = CONNECTIONS.get(&request.robot_id) {
            let net_info = conn
                .value()
                .send_instruction(Instruction::FetchNetwork {})
                .await;
            match net_info {
                Ok(info) => {
                    with_database(|db| {
                        db.write_network_info(&request.robot_id, &info)
                    })?
                    .await
                    .map_err(|err| {
                        GenericResponse::InternalError(PlainText(format!(
                            "Failed to write network info: {}",
                            err
                        )))
                    })?;
                    Ok(Json(fetch_network::FetchNetworkResponse {}))
                }
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

    #[oai(path = "/action/refresh_network_all", method = "post")]
    async fn refresh_network_all(
        &self,
    ) -> ApiResult<fetch_network::FetchNetworkResponse> {
        for conn in CONNECTIONS.iter() {
            let robot_id = conn.key().clone();
            let net_info = conn
                .value()
                .send_instruction(Instruction::FetchNetwork {})
                .await;
            match net_info {
                Ok(info) => {
                    with_database(|db| {
                        db.write_network_info(&robot_id, &info)
                    })?
                    .await
                    .map_err(|err| {
                        GenericResponse::InternalError(PlainText(format!(
                            "Failed to write network info for robot {}: {}",
                            robot_id, err
                        )))
                    })?;
                }
                Err(err) => {
                    log::error!(
                        "Failed to fetch network info from robot {}: {:?}",
                        robot_id,
                        err
                    );
                }
            }
        }
        Ok(Json(fetch_network::FetchNetworkResponse {}))
    }
}
