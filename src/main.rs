mod clone;
mod commit;
mod config;
mod download;
mod init;
mod utils;

use anyhow::anyhow;
use clap::{Parser, Subcommand};
use commit::command_commit;
use duct::cmd;

use crate::{
    clone::command_clone,
    commit::{command_amend, command_reset},
    download::command_download,
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
    Init {
        /// create a GitHub repo if given
        #[arg(short, long, alias = "gh")]
        github: bool,

        /// make the GitHub repository private if given
        #[arg(long)]
        private: bool,
    },

    /// Clone a repo from remote.
    Clone {
        /// The url, or full name of a repo ("owner/repo") to clone.
        url_or_fullname: String,
    },

    /// Create a commit
    #[command(alias = "cm")]
    Commit {
        /// Commit message. You can omit this for now, and it will prompt you for a message after choosing what to commit.
        message: Option<String>,
    },

    /// Download a repo without its version history.
    /// Learnt from github@psnszsn/degit-rs.
    #[command(aliases = ["dl", "down"])]
    Download { url_or_fullname: String },

    /// Push changes to the remote
    #[command(alias = "ps")]
    Push,

    /// Pull changes from the remote
    #[command(alias = "pl")]
    Pull,

    /// Amend the last commit
    #[command(alias = "am")]
    Amend,

    /// Reset the latest commit
    #[command(alias = "rs")]
    Reset,
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
        && config.default_host == "github.com"
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
        Commands::Init { github, private } => {
            if let Err(e) = command_init(&config, *github, *private) {
                error(&e.to_string());
            }
        }
        Commands::Clone { url_or_fullname } => {
            if let Err(e) = command_clone(&config, url_or_fullname) {
                error(&e.to_string());
            }
        }
        Commands::Commit { message } => {
            if let Err(e) = command_commit(message.clone()) {
                error(&e.to_string());
            }
        }
        Commands::Amend => {
            if let Err(e) = command_amend() {
                error(&e.to_string());
            }
        }
        Commands::Reset => {
            if let Err(e) = command_reset() {
                error(&e.to_string());
            }
        }
        Commands::Push => {
            println!("Changes pushed.");
        }
        Commands::Pull => {
            println!("Changes pulled.");
        }
        Commands::Download { url_or_fullname } => {
            if let Err(e) = command_download(&config, url_or_fullname) {
                error(&e.to_string());
            }
        }
    }
}
