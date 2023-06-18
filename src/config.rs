use serde::Deserialize;
use std::path::PathBuf;

const CONFIG_FILE: &str = "config.toml";

#[derive(Deserialize)]
pub struct Settings {
    pub discord_client: DcClientSettings,
    pub quote_file_path: String,
}

impl Settings {
    pub fn get() -> Result<Self, config::ConfigError> {
        let config = config::Config::builder()
            .add_source(config::File::from(PathBuf::from(CONFIG_FILE)))
            .build()?;

        config.try_deserialize()
    }
}

#[derive(Deserialize)]
pub struct DcClientSettings {
    pub token: String,
}
