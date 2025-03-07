use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub tg_token: String,
}
impl Settings {
    pub fn init() -> anyhow::Result<Self> {
        let base_path = std::env::current_dir()?;
        let file = base_path.join("config.toml");
        let config = config::Config::builder()
            .add_source(config::File::from(file).required(true))
            .add_source(config::Environment::with_prefix("PLANIFY"))
            .build()?;
        Ok(config.try_deserialize()?)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_init() {
        let settings = Settings::init().unwrap();
        assert!(!settings.tg_token.is_empty());
    }
}