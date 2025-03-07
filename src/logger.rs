use tracing::Level;

pub fn init(level: Level) -> anyhow::Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .pretty()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(false)
        .with_target(false)
        .with_max_level(level)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    tracing::info!("Это информационное сообщение");
    tracing::debug!("Это отладочное сообщение");
    tracing::warn!("Это предупреждающее сообщение");
    tracing::error!("Это сообщение об ошибке");
    Ok(())
}

#[cfg(test)]
mod tests {
    use tracing::Level;

    #[test]
    fn test_init() {
        let result = super::init(Level::WARN);
        assert!(result.is_ok());
    }
}