use poem_openapi::{
    OpenApi,
    param::Path,
    payload::{Json, PlainText},
};

use crate::{
    api::{ApiResult, GenericResponse},
    database::{robot::RobotIdent, with_database},
};

pub mod get_robot_network_stats;

pub struct StatsApi;

#[OpenApi]
impl StatsApi {
    #[oai(path = "/stats/robots", method = "get")]
    async fn get_registered_robots(&self) -> ApiResult<Vec<String>> {
        let robots = crate::database::with_database(|db| db.get_robots())?
            .await
            .map_err(|e| {
                poem::Error::from_string(
                    format!("Failed to fetch robots: {}", e),
                    poem::http::StatusCode::INTERNAL_SERVER_ERROR,
                )
            })?;
        Ok(Json(robots))
    }

    #[oai(path = "/stats/online_robots", method = "get")]
    async fn get_online_robots(&self) -> ApiResult<Vec<String>> {
        let online_robots: Vec<String> = crate::service::CONNECTIONS
            .iter()
            .map(|entry| entry.key().clone())
            .collect();
        Ok(Json(online_robots))
    }

    #[oai(path = "/stats/robot/:uuid", method = "get")]
    async fn get_robot(
        &self,
        Path(uuid): Path<String>,
    ) -> ApiResult<Option<RobotIdent>> {
        let robot =
            crate::database::with_database(|db| db.get_robot_by_id(&uuid))?
                .await
                .map_err(|e| {
                    poem::Error::from_string(
                        format!("Failed to fetch robot: {}", e),
                        poem::http::StatusCode::INTERNAL_SERVER_ERROR,
                    )
                })?;
        Ok(Json(robot))
    }

    #[oai(path = "/stats/robot/:uuid/network", method = "get")]
    async fn get_robot_network_stats(
        &self,
        Path(uuid): Path<String>,
    ) -> ApiResult<get_robot_network_stats::RobotNetworkStatsResponse> {
        let stats = with_database(|db| db.get_network_info(&uuid))?.await?;

        if let Some(network_info) = stats {
            Ok(Json(get_robot_network_stats::RobotNetworkStatsResponse {
                stats: network_info.info,
                last_updated: network_info.last_updated,
            }))
        } else {
            Err(GenericResponse::NotFound(PlainText(format!(
                "No network info found for robot with UUID: {}",
                uuid
            ))))
        }
    }
}
