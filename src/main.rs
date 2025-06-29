mod commit;
mod config;
mod init;
mod utils;

use anyhow::anyhow;
use clap::{Parser, Subcommand};
use commit::command_commit;
use duct::cmd;

use crate::{
    init::command_init,
    utils::{error, hint, warning},
};

#[derive(Parser)]
#[command(name = "hj")]
#[command(author = "Gao Junran <nebula2021@126.com>")]
#[command(version = "0.1")]
#[command(about = "Fast, opinionated version control experience.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new jj repository.
    Init,

    /// Create a commit
    #[command(alias = "cm")]
    Commit {
        /// Commit message. You can omit this for now, and it will prompt you for a message after choosing what to commit.
        message: Option<String>,
    },

    /// Push changes to the remote
    #[command(alias = "ps")]
    Push,

    /// Pull changes from the remote
    #[command(alias = "pl")]
    Pull,
}

fn check_jj_installed() -> anyhow::Result<()> {
    if cmd!("jj", "--version").read().is_err() {
        return Err(anyhow!(
            "jj is not installed or not found in PATH. Please install jj first."
        ));
    }
    Ok(())
}

fn check_gh_installed() -> anyhow::Result<()> {
    if cmd!("gh", "--version").read().is_err() {
        return Err(anyhow!(
            "gh is not installed or not found in PATH. Please install gh first."
        ));
    }
    Ok(())
}

fn main() {
    let config = config::AppConfig::from_env().unwrap_or_else(|err| {
            let location = dirs::config_dir().unwrap()
                .join("hj/config.toml");
            error(&err.to_string());
            hint(&format!("You can put your configuration in {}, or use environment variables prefixed with `HJ_`.", location.display()));
            std::process::exit(1)
        });
    let cli = Cli::parse();
    if let Err(e) = check_jj_installed() {
        error(&e.to_string());
        hint("https://jj-vcs.github.io/jj/latest/install-and-setup/");
        return;
    }
    if let Err(e) = check_gh_installed()
        && config.default_remote_host == "github.com"
        && config.check_gh
    {
        warning(&e.to_string());
        hint(
            "`gh` CLI brings convenience for GitHub operations. Ignore this if you don't use GitHub.",
        );
        hint("https://github.com/cli/cli#installation");
        hint("Set config key `check_gh` to false to disable this check.");
        println!();
    }
    match &cli.command {
        Commands::Init => {
            if let Err(e) = command_init(&config) {
                error(&e.to_string());
            }
        }
        Commands::Commit { message } => {
            if let Err(e) = command_commit(message.clone()) {
                error(&e.to_string());
            }
        }
        Commands::Push => {
            println!("Changes pushed.");
        }
        Commands::Pull => {
            println!("Changes pulled.");
        }
    }
}
