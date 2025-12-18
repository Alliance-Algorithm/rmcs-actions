use sqlx::prelude::FromRow;

use crate::database::Database;

#[derive(Debug, Clone, FromRow)]
pub struct RobotIdent {
    pub robot_id: String,
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

    pub async fn get_robot_by_id(
        &self,
        robot_id: &str,
    ) -> Result<Option<RobotIdent>, sqlx::Error> {
        let row = sqlx::query_as!(
            RobotIdent,
            "SELECT robot_id, mac, name, uuid FROM robots WHERE robot_id = ?",
            robot_id
        )
        .fetch_optional(&self.connection)
        .await?;
        Ok(row)
    }

    pub async fn register_robot(
        &self,
        robot_id: &str,
        mac_address: &str,
        name: &str,
        uuid: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "
                INSERT INTO robots
                (robot_id, mac, name, uuid) VALUES (?, ?, ?, ?)
                ON CONFLICT(uuid) DO UPDATE SET
                robot_id=excluded.robot_id, mac=excluded.mac, name=excluded.name
            ",
            robot_id,
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
            "SELECT robot_id, mac, name, uuid FROM robots
            WHERE name = ? AND mac = ? LIMIT 1",
            pattern,
            mac_address
        )
        .fetch_optional(&self.connection)
        .await?;
        Ok(row)
    }
}
