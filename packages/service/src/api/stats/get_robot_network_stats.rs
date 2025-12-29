use chrono::{DateTime, Utc};
use poem_openapi::Object;
use serde::{Deserialize, Serialize};

use crate::database::network::NetworkInfo;

#[derive(Debug, Clone, Serialize, Deserialize, Object)]
pub struct RobotNetworkStatsResponse {
    pub stats: NetworkInfo,
    pub last_updated: DateTime<Utc>,
}