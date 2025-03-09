mod schema;

use std::sync::Arc;
use schema::State;
use teloxide::Bot;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::*;
use tracing::instrument;
use crate::Storage;

pub struct TelegramClient {
    pub bot: Bot,
    pub storage: Arc<Storage>,
}
impl TelegramClient {
    #[instrument(skip_all)]
    pub fn new(token: &str, storage: Arc<Storage>) -> Self {
        let bot = Bot::new(token);
        Self {bot, storage}
    }
    #[instrument(skip(self))]
    pub async fn start(&self) {
        Dispatcher::builder(self.bot.clone(), schema::router())
            // Here you specify initial dependencies that all handlers will receive; they can be
            // database connections, configurations, and other auxiliary arguments. It is similar to
            // `actix_web::Extensions`.
            .dependencies(dptree::deps![InMemStorage::<State>::new(), self.storage.clone()])
            // If no handler succeeded to handle an update, this closure will be called.
            .default_handler(|upd| async move {
                log::warn!("Unhandled update: {:?}", upd);
            })
            // If the dispatcher fails for some reason, execute this handler.
            .error_handler(LoggingErrorHandler::with_custom_text(
                "An error has occurred in the dispatcher",
            ))
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;
    }
}