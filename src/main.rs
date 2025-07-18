mod clone;
mod commit;
mod config;
mod download;
mod init;
mod keepup;
mod log;
mod pull;
mod push;
mod switch;
mod tools;
mod upbase;
mod utils;

use std::env;

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
    log::command_log_all,
    pull::command_pull,
    push::command_push,
    switch::command_switch,
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
    /// Start version control experience!
    Init {
        /// Create a GitHub repo if given. You should have `gh` installed and logged in.
        #[arg(short, long, alias = "gh")]
        github: bool,

        /// Make the GitHub repository private if given
        #[arg(long)]
        private: bool,
    },

    /// Clone a repo from remote.
    Clone {
        /// The url, or full name of a repo ("owner/repo") to clone.
        source: String,
        /// The directory to clone into. By default is ./<repo-name>.
        destination: Option<String>,
    },

    /// Create a commit.
    #[command(alias = "cm")]
    Commit {
        /// Commit message. You can omit this for now, and it will prompt you for a message later.
        message: Option<String>,

        /// Run `hj push` after committing.
        #[arg(short, long)]
        push: bool,
    },

    /// Download a repo without its version history.
    #[command(aliases = ["dl", "down"])]
    Download {
        /// The url, or full name of a repo ("owner/repo") to download.
        source: String,
        /// Path to download the repo to. By default is `./<repo-name>`.
        destination: Option<String>,
        /// Entries (specified files or directories) to download. If not given, download the whole repo.
        #[arg(short, long)]
        entry: Vec<String>,
    },

    /// Push changes to the remote
    #[command(alias = "ps")]
    Push {
        /// The branches to push to. If not given, push the current branch (closest bookmark).
        branch: Vec<String>,

        /// Give revsets of changes to push. It will name the branch automatically.
        #[arg(short, long)]
        change: Vec<String>,

        /// Whether to keepup or not. (by default true)
        #[arg(short, long)]
        keepup: bool,

        /// Whether to pull before pushing. If given, the argument `branch` only accepts SINGLE branch.
        #[arg(short, long)]
        pull: bool,

        /// Whether to upbase before pushing.
        #[arg(short, long)]
        upbase: bool,
    },

    /// Pull changes from the remote.
    #[command(alias = "pl")]
    Pull {
        // Specify where our new work will be based on. Skip rebasing if not given.
        branch: Option<String>,
    },

    /// Amend from working copy to a commit (by default the latest one).
    #[command(alias = "am")]
    Amend {
        into: Option<String>,
        /// force amend (allow mutate the immutable commit)
        #[arg(short, long)]
        force: bool,
    },

    /// Reset from a commit (by default the latest one) to working copy.
    #[command(alias = "rs")]
    Reset {
        from: Option<String>,
        /// force reset (allow mutate the immutable commit)
        #[arg(short, long)]
        force: bool,
    },

    /// Rebase branches onto the trunk, which means updating the trunk and make all the other branches based on the NEW trunk.
    #[command(alias = "up")]
    Upbase {
        branch: Vec<String>,

        /// whether to fetch or not
        #[arg(short, long)]
        fetch: bool,
    },

    /// Keepup bookmarks to the latest commit.
    /// Often it runs internally in `push` or `switch` commands.
    /// If you move the working copy, it should be run manually.
    #[command(aliases = ["tug", "k"])]
    Keepup { branch: Vec<String> },

    /// Switch to a branch.
    #[command(alias = "sw")]
    Switch {
        branch: String,
        /// whether to keepup or not
        #[arg(short, long)]
        keepup: bool,
    },

    /// Log all commits.
    #[command(aliases = ["lg", "all"])]
    LogAll,
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

pub(crate) fn check_git_installed() -> anyhow::Result<()> {
    if cmd!("git", "--version").read().is_err() {
        return Err(anyhow!(
            "git is not installed or not found in PATH. Please install git first."
        ));
    }
    Ok(())
}

fn check_dot_git() -> bool {
    std::path::Path::new(".git").exists()
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
    if config.shortcut_branches.contains(&subcommand) {
        if let Err(e) = command_switch(&config, subcommand, true, check_dot_git()) {
            error(&e.to_string());
        }
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
        Commands::Amend { into, force } => {
            if let Err(e) = command_amend(into.clone(), *force) {
                error(&e.to_string());
            }
        }
        Commands::Reset { from, force } => {
            if let Err(e) = command_reset(from.clone(), *force) {
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
            source,
            destination,
            entry,
        } => {
            if let Err(e) =
                command_download(&config, source, destination.as_deref(), entry.to_vec())
            {
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
        Commands::Switch { branch, keepup } => {
            if let Err(e) = command_switch(&config, branch.clone(), *keepup, check_dot_git()) {
                error(&e.to_string());
            }
        }
        Commands::LogAll => {
            if let Err(e) = command_log_all(&config) {
                error(&e.to_string());
            }
        }
    }
}
