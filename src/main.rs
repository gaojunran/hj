mod clone;
mod commit;
mod config;
mod download;
mod init;
mod keepup;
mod pull;
mod push;
mod tools;
mod upbase;
mod utils;

use std::{env, iter};

use anyhow::anyhow;
use clap::{Parser, Subcommand};
use commit::command_commit;
use duct::cmd;

use crate::{
    clone::command_clone,
    commit::{command_amend, command_reset},
    download::command_download,
    init::command_init,
    keepup::command_keepup,
    pull::command_pull,
    push::command_push,
    upbase::command_upbase,
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
        source: String,
        /// The directory to clone into.
        destination: Option<String>,
    },

    /// Create a commit
    #[command(alias = "cm")]
    Commit {
        /// Commit message. You can omit this for now, and it will prompt you for a message after choosing what to commit.
        message: Option<String>,

        #[arg(short, long)]
        push: bool,
    },

    /// Download a repo without its version history.
    /// Learnt from github@psnszsn/degit-rs.
    #[command(aliases = ["dl", "down"])]
    Download {
        url_or_fullname: String,
        // the directory name to download the repo to.
        name: Option<String>,
    },

    /// Push changes to the remote
    #[command(alias = "ps")]
    Push {
        branch: Vec<String>,

        #[arg(short, long)]
        change: Vec<String>,

        #[arg(short, long)]
        keepup: bool,

        #[arg(short, long)]
        pull: bool,

        #[arg(short, long)]
        upbase: bool,
    },

    /// Pull changes from the remote
    #[command(alias = "pl")]
    Pull {
        // the branch to start working on / rebase on.
        branch: Option<String>,
    },

    /// Amend the last commit
    #[command(alias = "am")]
    Amend { into: Option<String> },

    /// Reset the latest commit
    #[command(alias = "rs")]
    Reset { from: Option<String> },

    /// Rebase branches onto the trunk
    #[command(alias = "up")]
    Upbase {
        branch: Vec<String>,

        /// whether to fetch or not
        #[arg(short, long)]
        fetch: bool,
    },

    /// Keepup bookmark(s) to the latest commit
    /// It should run internally in `push` or `switch` commands
    #[command(aliases = ["tug", "k"])]
    Keepup { branch: Vec<String> },
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
    let subcommand = env::args().nth(1).unwrap_or_else(|| {
        if let Err(e) = cmd!("jj").run() {
            error(&e.to_string());
        }
        std::process::exit(0);
    });
    if config.fallback_commands.contains(&subcommand) {
        if let Err(e) = cmd("jj", env::args().skip(1)).run() {
            error(&e.to_string());
        };
        std::process::exit(0);
    }
    let cli = Cli::parse();
    match &cli.command {
        Commands::Init { github, private } => {
            if let Err(e) = command_init(&config, *github, *private) {
                error(&e.to_string());
            }
        }
        Commands::Clone {
            source,
            destination,
        } => {
            if let Err(e) = command_clone(&config, source, destination.as_deref()) {
                error(&e.to_string());
            }
        }
        Commands::Commit { message, push } => {
            if let Err(e) = command_commit(&config, message.clone(), *push) {
                error(&e.to_string());
            }
        }
        Commands::Amend { into } => {
            if let Err(e) = command_amend(into.clone()) {
                error(&e.to_string());
            }
        }
        Commands::Reset { from } => {
            if let Err(e) = command_reset(from.clone()) {
                error(&e.to_string());
            }
        }
        Commands::Push {
            branch,
            change,
            keepup,
            pull,
            upbase,
        } => {
            if let Err(e) = command_push(&config, branch, change, *keepup, *pull, *upbase) {
                error(&e.to_string());
            }
        }
        Commands::Pull { branch } => {
            if let Err(e) = command_pull(&config, branch.clone()) {
                error(&e.to_string());
            }
        }
        Commands::Download {
            url_or_fullname,
            name,
        } => {
            if let Err(e) = command_download(&config, url_or_fullname, name.as_deref()) {
                error(&e.to_string());
            }
        }
        Commands::Upbase { branch, fetch } => {
            if let Err(e) = command_upbase(&config, branch, *fetch) {
                error(&e.to_string());
            }
        }
        Commands::Keepup { branch } => {
            if let Err(e) = command_keepup(&config, branch) {
                error(&e.to_string());
            }
        }
    }
}
