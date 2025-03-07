mod settings;

pub use settings::Settings;
use tracing::instrument;
pub mod logger;
pub mod models;
mod storage;
pub use storage::Storage;

use anyhow::Result;

pub struct App {
    pub settings: Settings,
    pub storage: Storage,
}
impl App {
    #[instrument]
    pub async fn new(log_level: tracing::Level) -> Result<Self> {
        logger::init(log_level);
        let settings = Settings::init()?;
        let storage = Storage::new().await?;
        let res = App {
            settings,
            storage,
        };
        Ok(res)
    }
    pub async fn run(&self) -> Result<()> {
        Ok(())
    }
    pub async fn shutdown(&self) -> Result<()> {
        self.storage.close().await;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_new_app() -> Result<()> {
        let app = App::new(tracing::Level::DEBUG).await?;
        app.shutdown().await?;
        Ok(())
    }
}
