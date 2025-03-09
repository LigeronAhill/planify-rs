mod settings;

use std::sync::Arc;
pub use settings::Settings;
use tracing::instrument;
pub mod logger;
pub mod models;
mod storage;
pub use storage::Storage;
mod telegram;
use telegram::TelegramClient;

use anyhow::Result;

pub struct App {
    pub tg_client: TelegramClient,
}
impl App {
    #[instrument]
    pub async fn new(log_level: tracing::Level) -> Result<Self> {
        logger::init(log_level);
        let settings = Settings::init()?;
        let tg_token = settings.tg_token;
        let storage = Arc::new(Storage::new().await?);
        let tg_client = TelegramClient::new(&tg_token, storage);
        let res = App {
            tg_client,
        };
        Ok(res)
    }
    pub async fn run(&self) -> Result<()> {
        self.tg_client.start().await;
        Ok(())
    }
    pub async fn shutdown(&self) -> Result<()> {
        self.tg_client.storage.close().await;
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
