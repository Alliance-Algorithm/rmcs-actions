use std::sync::OnceLock;

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
            "CREATE TABLE IF NOT EXISTS robots (id INTEGER PRIMARY KEY, name TEXT NOT NULL)",
        )
        .execute(&self.connection)
        .await?;
        Ok(())
    }
}

pub static DATABASE: OnceLock<Database> = OnceLock::new();

pub fn get_database() -> &'static Database {
    DATABASE.get().expect("Database not initialized")
}
