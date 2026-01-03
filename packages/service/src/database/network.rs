use chrono::{DateTime, Utc};
use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use serde_with::{DefaultOnNull, serde_as};

use crate::database::Database;

pub type NetworkInfo = Vec<NetworkInfoItem>;

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, Object)]
pub struct NetworkInfoItem {
    pub index: i32,
    pub mtu: i32,
    pub name: String,
    pub hardware_addr: String,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub flags: Vec<String>,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub addrs: Vec<Addr>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Object)]
pub struct Addr {
    pub addr: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Object)]
pub struct NetworkInfoRow {
    pub info: NetworkInfo,
    pub last_updated: DateTime<Utc>,
}

impl Database {
    pub async fn write_network_info(
        &self,
        uuid: &str,
        info: &NetworkInfo,
    ) -> anyhow::Result<()> {
        let info_json = serde_json::to_string(info)?;
        sqlx::query!(
            "INSERT OR REPLACE INTO network_info (robot_uuid, info) VALUES (?, ?)",
            uuid,
            info_json,
        )
        .execute(&self.connection)
        .await?;
        Ok(())
    }

    pub async fn get_network_info(
        &self,
        uuid: &str,
    ) -> anyhow::Result<Option<NetworkInfoRow>> {
        let record = sqlx::query!(
            "SELECT info, last_updated FROM network_info WHERE robot_uuid = ?",
            uuid
        )
        .fetch_optional(&self.connection)
        .await?;
        if let Some(row) = record {
            let info: NetworkInfo = serde_json::from_str(&row.info)?;
            Ok(Some(NetworkInfoRow {
                info,
                last_updated: row.last_updated.and_utc(),
            }))
        } else {
            Ok(None)
        }
    }
}
