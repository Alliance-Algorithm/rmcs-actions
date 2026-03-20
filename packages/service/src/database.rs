use std::{str::FromStr, sync::OnceLock};

use sqlx::sqlite::SqliteConnectOptions;

pub mod network;
pub mod robot;

pub struct Database {
    connection: sqlx::SqlitePool,
}

const CREATE_NETWORK_INFO_TABLE_SQL: &str = "
    CREATE TABLE IF NOT EXISTS network_info (
        robot_uuid   TEXT PRIMARY KEY NOT NULL,
        info         TEXT NOT NULL,
        last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
        FOREIGN KEY (robot_uuid) REFERENCES robots(uuid) ON DELETE CASCADE
    )
";

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let connect_options =
            SqliteConnectOptions::from_str(database_url)?.foreign_keys(true);
        let connection =
            sqlx::SqlitePool::connect_with(connect_options).await?;
        Ok(Self { connection })
    }

    pub async fn init(&self) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "
                CREATE TABLE IF NOT EXISTS robots (
                    uuid TEXT PRIMARY KEY NOT NULL,
                    name TEXT NOT NULL,
                    mac TEXT NOT NULL
                )
            ",
        )
        .execute(&self.connection)
        .await?;

        self.init_network_info_table().await?;

        Ok(())
    }

    async fn init_network_info_table(&self) -> Result<(), sqlx::Error> {
        let network_info_table_exists: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = 'network_info'",
        )
        .fetch_one(&self.connection)
        .await?;

        if network_info_table_exists == 0 {
            sqlx::query(CREATE_NETWORK_INFO_TABLE_SQL)
                .execute(&self.connection)
                .await?;
            return Ok(());
        }

        let network_info_has_robot_fk: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM pragma_foreign_key_list('network_info')
             WHERE \"table\" = 'robots' AND \"from\" = 'robot_uuid' AND \"to\" = 'uuid'",
        )
        .fetch_one(&self.connection)
        .await?;

        if network_info_has_robot_fk == 0 {
            let mut transaction = self.connection.begin().await?;
            sqlx::query(
                "ALTER TABLE network_info RENAME TO network_info_legacy",
            )
            .execute(&mut *transaction)
            .await?;
            sqlx::query(CREATE_NETWORK_INFO_TABLE_SQL)
                .execute(&mut *transaction)
                .await?;
            sqlx::query(
                "INSERT INTO network_info (robot_uuid, info, last_updated)
                 SELECT robot_uuid, info, last_updated FROM network_info_legacy",
            )
            .execute(&mut *transaction)
            .await?;
            sqlx::query("DROP TABLE network_info_legacy")
                .execute(&mut *transaction)
                .await?;
            transaction.commit().await?;
        }

        Ok(())
    }
}

pub static DATABASE: OnceLock<Database> = OnceLock::new();

pub fn get_database() -> anyhow::Result<&'static Database> {
    DATABASE.get().ok_or_else(|| {
        anyhow::anyhow!("Database not initialized. Make sure to call Database::new and set DATABASE.")
    })
}

pub fn with_database<F, R>(f: F) -> anyhow::Result<R>
where
    F: FnOnce(&'static Database) -> R,
{
    DATABASE.get().map(f).ok_or_else(|| {
        anyhow::anyhow!("Database not initialized. Make sure to call Database::new and set DATABASE.")
    })
}
