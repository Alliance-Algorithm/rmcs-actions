use std::time::Duration;

use poem_openapi::{
    OpenApi,
    payload::{Json, PlainText},
};
use tokio::time::timeout;

use crate::{
    api::{AnyDeserialize, ApiResult, GenericResponse},
    database::with_database,
    service::{CONNECTIONS, instructions::Instruction},
};

pub mod fetch_network;
pub mod set_robot_name;
pub mod update_binary;

const UPDATE_BINARY_TIMEOUT: Duration = Duration::from_secs(60);

fn parse_update_binary_response(
    info: serde_json::Value,
) -> update_binary::UpdateBinaryResponse {
    let status = info
        .get("status")
        .and_then(|v| v.as_str())
        .unwrap_or("error")
        .to_string();
    let message = info
        .get("message")
        .and_then(|v| v.as_str())
        .unwrap_or("invalid response: missing fields")
        .to_string();

    update_binary::UpdateBinaryResponse { status, message }
}

fn update_binary_error_response(
    message: impl Into<String>,
) -> update_binary::UpdateBinaryResponse {
    update_binary::UpdateBinaryResponse {
        status: "error".to_string(),
        message: message.into(),
    }
}

fn update_binary_instruction_failure_response(
    err: impl std::fmt::Display,
) -> update_binary::UpdateBinaryResponse {
    update_binary_error_response(format!("instruction failed: {}", err))
}

fn update_binary_timeout_response() -> update_binary::UpdateBinaryResponse {
    update_binary_error_response(format!(
        "instruction timed out after {} seconds",
        UPDATE_BINARY_TIMEOUT.as_secs()
    ))
}

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
            with_database(|db| {
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

    #[oai(path = "/action/update_binary", method = "post")]
    async fn update_binary(
        &self,
        request: Json<update_binary::UpdateBinaryRequest>,
    ) -> ApiResult<update_binary::UpdateBinaryResponse> {
        if let Some(conn) = CONNECTIONS.get(&request.robot_id) {
            let result = timeout(
                UPDATE_BINARY_TIMEOUT,
                conn.value().send_instruction::<serde_json::Value>(
                    Instruction::UpdateBinary {
                        artifact_url: request.artifact_url.clone(),
                    },
                ),
            )
            .await;
            match result {
                Ok(Ok(info)) => Ok(Json(parse_update_binary_response(info))),
                Ok(Err(err)) => {
                    log::error!(
                        "Failed to update binary on robot {}: {:?}",
                        request.robot_id,
                        err
                    );
                    Ok(Json(update_binary_instruction_failure_response(err)))
                }
                Err(_) => {
                    log::error!(
                        "Timed out updating binary on robot {} after {} seconds",
                        request.robot_id,
                        UPDATE_BINARY_TIMEOUT.as_secs()
                    );
                    Ok(Json(update_binary_timeout_response()))
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

    #[oai(path = "/action/update_binary_all", method = "post")]
    async fn update_binary_all(
        &self,
        request: Json<update_binary::UpdateBinaryAllRequest>,
    ) -> ApiResult<update_binary::UpdateBinaryAllResponse> {
        let mut results = Vec::new();
        let mut has_failure = false;

        for conn in CONNECTIONS.iter() {
            let robot_id = conn.key().clone();
            let result = timeout(
                UPDATE_BINARY_TIMEOUT,
                conn.value().send_instruction::<serde_json::Value>(
                    Instruction::UpdateBinary {
                        artifact_url: request.artifact_url.clone(),
                    },
                ),
            )
            .await;
            match result {
                Ok(Ok(info)) => {
                    let response = parse_update_binary_response(info);
                    if response.status != "post_update" {
                        has_failure = true;
                    }
                    results.push(update_binary::RobotUpdateResult {
                        robot_id,
                        status: response.status,
                        message: response.message,
                    });
                }
                Ok(Err(err)) => {
                    has_failure = true;
                    log::error!(
                        "Failed to update binary on robot {}: {:?}",
                        robot_id,
                        err
                    );
                    let response =
                        update_binary_instruction_failure_response(err);
                    results.push(update_binary::RobotUpdateResult {
                        robot_id,
                        status: response.status,
                        message: response.message,
                    });
                }
                Err(_) => {
                    has_failure = true;
                    log::error!(
                        "Timed out updating binary on robot {} after {} seconds",
                        robot_id,
                        UPDATE_BINARY_TIMEOUT.as_secs()
                    );
                    let response = update_binary_timeout_response();
                    results.push(update_binary::RobotUpdateResult {
                        robot_id,
                        status: response.status,
                        message: response.message,
                    });
                }
            }
        }

        let overall_status = if has_failure { "partial_failure" } else { "ok" };

        Ok(Json(update_binary::UpdateBinaryAllResponse {
            status: overall_status.to_string(),
            results,
        }))
    }
}
