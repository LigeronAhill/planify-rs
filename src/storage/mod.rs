mod users;
mod tasks;
use sqlx::{ConnectOptions, Pool, Sqlite, SqlitePool};
use anyhow::Result;
use sqlx::sqlite::SqliteConnectOptions;

pub struct Storage {
    pool: Pool<Sqlite>
}
impl Storage {
    pub async fn new() -> Result<Self> {
        let base_path = std::env::current_dir()?;
        let storage_dir = base_path.join("storage");
        let file_path = storage_dir.join("storage.db");
        let test_path = storage_dir.join("test.db");
        let options = if !cfg!(test) {
            SqliteConnectOptions::new()
                .log_statements(log::LevelFilter::Info)
                .filename(file_path)
                .create_if_missing(true)
        } else {
            SqliteConnectOptions::new()
                .log_statements(log::LevelFilter::Debug)
                .filename(test_path)
                .create_if_missing(true)
        };
        let pool = SqlitePool::connect_with(options).await?;
        sqlx::migrate!("storage/migrations").run(&pool).await?;
        Ok(Self { pool })
    }
    pub async fn close(&self) {
        self.pool.close().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_new() -> anyhow::Result<()> {
        let storage = Storage::new().await?;
        storage.close().await;
        Ok(())
    }
}