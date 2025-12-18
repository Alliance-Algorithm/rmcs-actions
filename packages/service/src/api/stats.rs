use poem_openapi::{OpenApi, payload::Json};

use crate::{api::ApiResult, database::robot::RobotIdent};

pub struct StatsApi;

#[OpenApi]
impl StatsApi {
    /// This endpoint will return all registered robots' basic statistics.
    #[oai(path = "/stats/robots", method = "get")]
    async fn get_registered_robots(&self) -> ApiResult<Vec<RobotIdent>> {
        let db = crate::database::get_database();
        let robots = db.get_robots().await.map_err(|e| {
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
}
