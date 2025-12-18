use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::database::Database;

#[derive(Debug, Clone, FromRow, Object, Serialize, Deserialize)]
pub struct RobotIdent {
    pub mac: String,
    pub name: String,
    pub uuid: String,
}

impl Database {
    pub async fn get_robot_names(&self) -> Result<Vec<String>, sqlx::Error> {
        let rows = sqlx::query!("SELECT name FROM robots")
            .fetch_all(&self.connection)
            .await?;
        Ok(rows.into_iter().map(|row| row.name).collect())
    }

    pub async fn get_robots(&self) -> Result<Vec<RobotIdent>, sqlx::Error> {
        let rows =
            sqlx::query_as!(RobotIdent, "SELECT mac, name, uuid FROM robots")
                .fetch_all(&self.connection)
                .await?;
        Ok(rows)
    }

    pub async fn get_robot_by_id(
        &self,
        uuid: &str,
    ) -> Result<Option<RobotIdent>, sqlx::Error> {
        let row = sqlx::query_as!(
            RobotIdent,
            "SELECT mac, name, uuid FROM robots WHERE uuid = ?",
            uuid
        )
        .fetch_optional(&self.connection)
        .await?;
        Ok(row)
    }

    pub async fn register_robot(
        &self,
        mac_address: &str,
        name: &str,
        uuid: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "
                INSERT INTO robots
                (mac, name, uuid) VALUES (?, ?, ?)
                ON CONFLICT(uuid) DO UPDATE SET
                mac=excluded.mac, name=excluded.name
            ",
            mac_address,
            name,
            uuid
        )
        .execute(&self.connection)
        .await?;
        Ok(())
    }

    pub async fn fuzz_search_by_name(
        &self,
        username: &str,
        mac_address: &str,
    ) -> Result<Option<RobotIdent>, sqlx::Error> {
        let pattern = format!("%{}%", username);
        let row = sqlx::query_as!(
            RobotIdent,
            "SELECT mac, name, uuid FROM robots
            WHERE name = ? AND mac = ? LIMIT 1",
            pattern,
            mac_address
        )
        .fetch_optional(&self.connection)
        .await?;
        Ok(row)
    }
}
