use config::{Config, ConfigError, Environment, File};
use dirs::config_dir;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub check_gh: bool,
    pub default_host: String,
    pub init_config: InitConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InitConfig {
    pub default_remote_name: String,
    pub default_branch: String,
    pub create_github_repo: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            check_gh: true,
            default_host: "github.com".to_string(),
            init_config: InitConfig {
                default_remote_name: "origin".to_string(),
                default_branch: "main".to_string(),
                create_github_repo: false,
            },
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
