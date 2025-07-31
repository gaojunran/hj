use config::{Config, ConfigError, Environment, File};
use dirs::{config_dir, home_dir};
use std::{env, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub check_gh: bool,
    pub default_host: String,
    pub fallback_commands: Vec<String>,
    pub shortcut_branches: Vec<String>,
    pub always_colocate: bool,
    pub init_config: InitConfig,
    pub push_config: PushConfig,
    pub upbase_config: UpbaseConfig,
    pub switch_config: SwitchConfig,
    pub open_config: OpenConfig,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenConfig {
    pub editor: Option<String>,
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
                "desc", // describe
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
                "st", // status
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
            always_colocate: false,
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
            open_config: OpenConfig { editor: None },
        }
    }
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        // default config location: ~/.config/hj/config.toml
        let xdg = env::var("XDG_CONFIG_HOME").ok();
        let global_config_path = match xdg {
            Some(xdg) => PathBuf::from(xdg).join("hj/config.toml"),
            None => home_dir().unwrap().join(".config/hj/config.toml"),
        };
        // println!("global config path: {:?}", global_config_path);
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
            .add_source(
                Environment::with_prefix("HJ")
                    .prefix_separator("__")
                    .separator("__"),
            );

        let config = builder.build()?;
        config.try_deserialize()
    }
}
