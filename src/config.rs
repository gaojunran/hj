use config::{Config, ConfigError, Environment, File};
use dirs::config_dir;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub default_branch: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            default_branch: "main".to_string(),
        }
    }
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        // default config location: ~/.config/hj/config.toml
        let mut config_path = PathBuf::new();
        if let Some(mut base) = config_dir() {
            base.push("hj/config.toml");
            config_path = base;
        }

        let default = AppConfig::default();

        // load config
        let builder = Config::builder()
            // load file (if exists)
            .add_source(Config::try_from(&default)?)
            .add_source(File::from(config_path).required(false))
            // load environment variables with prefix HJ_
            .add_source(Environment::with_prefix("HJ").separator("_"));

        let config = builder.build()?;
        config.try_deserialize()
    }
}
