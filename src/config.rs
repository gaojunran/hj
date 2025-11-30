use config::{Config, ConfigError, Environment, File};
use dirs::home_dir;
use std::{env, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub check_gh: bool,
    // pub hosts: Vec<String>,
    pub shortcut_branches: Vec<String>,
    pub always_colocate: bool,
    pub hooks: HookConfig,
    pub clone: CloneConfig,
    pub init: InitConfig,
    pub push: PushConfig,
    pub upbase: UpbaseConfig,
    // pub switch: SwitchConfig,
    pub open: OpenConfig,
    pub keepup: KeepupConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CloneConfig {
    pub default_host: String,
    pub default_user: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InitConfig {
    pub default_remote_name: String,
    pub default_branch: String,
    pub create_github_repo: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PushConfig {
    /// do not keepup by default
    pub still: bool,
    pub pull: bool,
    pub upbase: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpbaseConfig {
    pub fetch: bool,
}

// #[derive(Debug, Deserialize, Serialize)]
// pub struct SwitchConfig {
//     pub keepup: bool,
// }

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenConfig {
    pub editor: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct KeepupConfig {
    pub avoid_trunk: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HookConfig {
    pub use_just: bool,
    #[serde(default)]
    pub ignore_git_hooks: Vec<String>,
    pub pre_commit: Option<String>,
    pub post_commit: Option<String>,
    pub pre_push: Option<String>,
    pub post_push: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            check_gh: true,
            shortcut_branches: ["main", "master", "trunk", "dev"]
                .iter()
                .map(|item| item.to_string())
                .collect(),
            always_colocate: false,
            clone: CloneConfig {
                default_host: "github.com".to_string(),
                default_user: None,
            },
            init: InitConfig {
                default_remote_name: "origin".to_string(),
                default_branch: "main".to_string(),
                create_github_repo: false,
            },
            push: PushConfig {
                still: false,
                pull: false,
                upbase: false,
            },
            // switch: SwitchConfig { keepup: true },
            upbase: UpbaseConfig { fetch: true },
            open: OpenConfig { editor: None },
            keepup: KeepupConfig { avoid_trunk: false },
            hooks: HookConfig {
                use_just: false,
                ignore_git_hooks: Vec::new(),
                pre_commit: None,
                post_commit: None,
                pre_push: None,
                post_push: None,
            },
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

        // Recursively search for hj.toml upwards from current directory
        let local_config_path = Self::find_local_config();

        let default = AppConfig::default();

        // load config
        let mut builder = Config::builder()
            // load file (if exists)
            .add_source(Config::try_from(&default)?)
            .add_source(File::from(global_config_path).required(false));

        // Add local config if found
        if let Some(path) = local_config_path {
            builder = builder.add_source(File::from(path).required(false));
        }

        let builder = builder
            // load environment variables with prefix HJ_
            .add_source(
                Environment::with_prefix("HJ")
                    .prefix_separator("__")
                    .separator("__"),
            );

        let config = builder.build()?;
        config.try_deserialize()
    }

    /// Recursively search for hj.toml upwards from the current directory
    fn find_local_config() -> Option<PathBuf> {
        let mut current_dir = env::current_dir().ok()?;

        loop {
            let config_path = current_dir.join("hj.toml");
            if config_path.exists() {
                return Some(config_path);
            }

            // Move to parent directory
            if !current_dir.pop() {
                // Reached the root directory
                break;
            }
        }

        None
    }
}
