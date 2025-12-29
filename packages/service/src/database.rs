use std::sync::OnceLock;

pub mod network;
pub mod robot;

pub struct Database {
    connection: sqlx::SqlitePool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let connection = sqlx::SqlitePool::connect(database_url).await?;
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
