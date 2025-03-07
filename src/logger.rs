use tracing::Level;

pub fn init(level: Level) {
    let subscriber = tracing_subscriber::fmt()
        .pretty()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(false)
        .with_target(false)
        .with_max_level(level)
        .finish();

    if let Err(e) = tracing::subscriber::set_global_default(subscriber) {
        tracing::error!("Ошибка инициализации глобального журнала: {e:?}");
    }

    tracing::info!("Это информационное сообщение");
    tracing::debug!("Это отладочное сообщение");
    tracing::warn!("Это предупреждающее сообщение");
    tracing::error!("Это сообщение об ошибке");
}

#[cfg(test)]
mod tests {
    use tracing::Level;
    use super::*;

    #[test]
    fn test_init() {
        init(Level::WARN);
    }
}