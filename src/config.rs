use config::{Config, ConfigError, Environment, File};
use dirs::config_dir;
use std::{env, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub check_gh: bool,
    pub default_host: String,
    pub fallback_commands: Vec<String>,
    pub shortcut_branches: Vec<String>,
    pub init_config: InitConfig,
    pub push_config: PushConfig,
    pub upbase_config: UpbaseConfig,
    pub switch_config: SwitchConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InitConfig {
    pub default_remote_name: String,
    pub default_branch: String,
    pub create_github_repo: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PushConfig {
    /// when pushing a branch, whether to keep up the specific bookmark to `@-` or not.
    pub keepup: bool,
    pub pull: bool,
    pub upbase: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpbaseConfig {
    pub fetch: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SwitchConfig {
    pub keepup: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            check_gh: true,
            default_host: "github.com".to_string(),
            fallback_commands: [
                "abandon",
                "absorb",
                "backout",
                "bookmark",
                "config",
                "debug",
                "describe",
                "diff",
                "diffedit",
                "duplicate",
                "edit",
                "evolog",
                "file",
                "fix",
                "git",
                "interdiff",
                "log",
                "new",
                "next",
                "operation",
                "parallelize",
                "prev",
                "rebase",
                "resolve",
                "restore",
                "revert",
                "root",
                "run",
                "show",
                "sign",
                "simplify-parents",
                "sparse",
                "split",
                "squash",
                "status",
                "tag",
                "undo",
                "unsign",
                "util",
                "version",
                "workspace",
            ]
            .iter()
            .map(|item| item.to_string())
            .collect(),
            shortcut_branches: ["main", "master", "trunk", "dev"]
                .iter()
                .map(|item| item.to_string())
                .collect(),
            init_config: InitConfig {
                default_remote_name: "origin".to_string(),
                default_branch: "main".to_string(),
                create_github_repo: false,
            },
            push_config: PushConfig {
                keepup: true,
                pull: false,
                upbase: false,
            },
            switch_config: SwitchConfig { keepup: true },
            upbase_config: UpbaseConfig { fetch: true },
        }
    }
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        // default config location: ~/.config/hj/config.toml
        let mut global_config_path = PathBuf::new();
        if let Some(mut base) = config_dir() {
            base.push("hj/config.toml");
            global_config_path = base;
        }
        let mut local_config_path = PathBuf::new();
        if let Ok(mut base) = env::current_dir() {
            base.push("hj.toml");
            local_config_path = base;
        }

        let default = AppConfig::default();

        // load config
        let builder = Config::builder()
            // load file (if exists)
            .add_source(Config::try_from(&default)?)
            .add_source(File::from(global_config_path).required(false))
            .add_source(File::from(local_config_path).required(false))
            // load environment variables with prefix HJ_
            .add_source(Environment::with_prefix("HJ").separator("_"));

        let config = builder.build()?;
        config.try_deserialize()
    }
}
