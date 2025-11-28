mod clone;
mod commit;
mod config;
mod download;
mod fallback;
mod fetch;
mod hook;
mod init;
mod keepup;
mod log;
mod open;
mod pull;
mod push;
mod switch;
mod tools;
mod upbase;
mod utils;

use anyhow::{Result, anyhow};
use clap::{Parser, Subcommand};
use commit::command_commit;
use duct::cmd;

use crate::{
    clone::command_clone,
    commit::{command_amend, command_reset, command_throw},
    download::command_download,
    fallback::handle_fallback_command,
    fetch::command_fetch,
    init::command_init,
    keepup::command_keepup,
    log::{command_log_all, command_log_wip},
    open::command_open,
    pull::command_pull,
    push::command_push,
    switch::command_switch,
    upbase::command_upbase,
    utils::{error, hint},
};

#[derive(Parser)]
#[command(name = "hj")]
#[command(author = "Gao Junran <nebula2021@126.com>")]
#[command(about = "Fast, opinionated version control experience.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start version control experience!
    #[command(alias = "in")]
    Init {
        /// Create a GitHub repo if given. You should have `gh` installed and logged in.
        #[arg(short, long, alias = "gh")]
        github: bool,

        /// Make the GitHub repository private if given
        #[arg(long)]
        private: bool,

        /// Whether to colocate or not.
        #[arg(short, long, alias = "git")]
        colocate: bool,
    },

    /// Clone a repo from remote.
    #[command(alias = "cl")]
    Clone {
        /// The url, or full name of a repo ("owner/repo") to clone.
        source: String,
        /// The directory to clone into. By default is ./<repo-name>.
        destination: Option<String>,

        /// Whether to colocate or not.
        #[arg(short, long, alias = "git")]
        colocate: bool,

        /// Fork a github repo and clone your fork
        #[arg(short, long)]
        fork: bool,
    },

    /// Create a commit.
    #[command(alias = "cm")]
    Commit {
        /// Commit message. You can omit this for now, and it will prompt you for a message later.
        message: Option<String>,

        /// Run `hj push` after committing.
        #[arg(short, long)]
        push: bool,

        /// Abandon uncommitted changes.
        #[arg(short, long)]
        abandon: bool,

        #[arg(long)]
        no_pre_hook: bool,

        #[arg(long)]
        no_post_hook: bool,
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

        /// Prevent `keepup`ing or not.
        #[arg(short, long)]
        still: bool,

        /// Whether to pull before pushing. If given, the argument `branch` only accepts SINGLE branch.
        #[arg(short, long)]
        pull: bool,

        /// Whether to upbase before pushing.
        #[arg(short, long)]
        upbase: bool,

        #[arg(long)]
        no_pre_hook: bool,

        #[arg(long)]
        no_post_hook: bool,
    },

    /// Pull changes from the remote.
    #[command(alias = "pl")]
    Pull {
        // Specify where our new work will be based on. Skip rebasing if not given.
        branch: Option<String>,
    },

    /// Fetch changes from remote (and track bookmarks). Can be shortened as `f`.
    #[command(alias = "f")]
    Fetch {
        /// The branches to fetch (will be tracked and passed to `jj git fetch --bookmark`).
        branch: Vec<String>,
    },

    /// Amend from working copy to a commit (by default the latest one).
    /// If using with EDIT workflow, means move some changes OUT of the editing commit.
    #[command(aliases = &["am", "out"])]
    Amend {
        into: Option<String>,

        /// Run `hj push` after amending.
        #[arg(short, long)]
        push: bool,

        /// force amend (allow mutate the immutable commit)
        #[arg(short, long)]
        force: bool,

        #[arg(long)]
        no_pre_hook: bool,

        #[arg(long)]
        no_post_hook: bool,
    },

    /// Reset from a commit (by default the latest one) to working copy.
    /// If using with EDIT workflow, means move some changes INTO the editing commit.
    #[command(aliases = &["rs", "in"])]
    Reset {
        from: Option<String>,

        /// Run `hj push` after resetting.
        #[arg(short, long)]
        push: bool,

        /// force reset (allow mutate the immutable commit)
        #[arg(short, long)]
        force: bool,
    },

    /// Pick changes from a commit (by default working copy) and throw them away.
    #[command(alias = "th")]
    Throw {
        from: Option<String>,
        /// force throw (allow mutate the immutable commit)
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
        /// The branches to switch from (we will keepup them)
        keepup_branch: Vec<String>,
        /// The branch to switch to.
        dest_branch: String,
    },

    /// Log all commits.
    #[command(aliases = ["all"])]
    LogAll,

    /// Log all wip(working in progress) commits.
    #[command(aliases = ["wip"])]
    LogWip {
        #[arg(short, long)]
        patch: bool,
    },

    /// Open root with your default editor.
    #[command(alias = "o")]
    Open {
        /// open remote url
        remote: Option<String>,
    },

    /// External subcommands (fallback to jj)
    #[command(external_subcommand)]
    External(Vec<String>),
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
            hint(&format!("You can put your configuration in {}, or use environment variables prefixed with `HJ__`.", location.display()));
            std::process::exit(1)
        });
    if let Err(e) = check_jj_installed() {
        error(&e.to_string());
        hint("https://jj-vcs.github.io/jj/latest/install-and-setup/");
        return;
    }

    let cli = Cli::parse();

    // If no subcommand is provided, fallback to jj without arguments
    let Some(command) = &cli.command else {
        use duct::cmd;
        if let Err(e) = cmd("jj", &[] as &[&str]).run() {
            error(&format!("Failed to execute jj: {}", e));
        }
        return;
    };

    match command {
        Commands::Init {
            github,
            private,
            colocate,
        } => {
            if let Err(e) = command_init(&config, *github, *private, *colocate) {
                error(&e.to_string());
            }
        }
        Commands::Clone {
            source,
            destination,
            colocate,
            fork,
        } => {
            if let Err(e) = command_clone(&config, source, destination.as_deref(), *colocate, *fork)
            {
                error(&e.to_string());
            }
        }
        Commands::Commit {
            message,
            push,
            abandon,
            no_pre_hook,
            no_post_hook,
        } => {
            if let Err(e) = command_commit(
                &config,
                message.clone(),
                *push,
                *abandon,
                *no_pre_hook,
                *no_post_hook,
            ) {
                error(&e.to_string());
            }
        }
        Commands::Amend {
            into,
            push,
            force,
            no_pre_hook,
            no_post_hook,
        } => {
            if let Err(e) = command_amend(
                &config,
                into.clone(),
                *push,
                *force,
                *no_pre_hook,
                *no_post_hook,
            ) {
                error(&e.to_string());
            }
        }
        Commands::Reset { from, push, force } => {
            if let Err(e) = command_reset(&config, from.clone(), *push, *force) {
                error(&e.to_string());
            }
        }
        Commands::Push {
            branch,
            change,
            still,
            pull,
            upbase,
            no_pre_hook,
            no_post_hook,
        } => {
            if let Err(e) = command_push(
                &config,
                branch,
                change,
                *still,
                *pull,
                *upbase,
                *no_pre_hook,
                *no_post_hook,
            ) {
                error(&e.to_string());
            }
        }
        Commands::Pull { branch } => {
            if let Err(e) = command_pull(&config, branch.clone()) {
                error(&e.to_string());
            }
        }
        Commands::Fetch { branch } => {
            if let Err(e) = command_fetch(&config, branch.clone()) {
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
        Commands::Switch {
            keepup_branch,
            dest_branch,
        } => {
            if let Err(e) =
                command_switch(&config, keepup_branch, dest_branch.clone(), check_dot_git())
            {
                error(&e.to_string());
            }
        }
        Commands::LogAll => {
            if let Err(e) = command_log_all(&config) {
                error(&e.to_string());
            }
        }
        Commands::LogWip { patch } => {
            if let Err(e) = command_log_wip(&config, *patch) {
                error(&e.to_string());
            }
        }
        Commands::Open { remote } => {
            if let Err(e) = command_open(&config, remote.clone()) {
                error(&e.to_string());
            }
        }
        Commands::Throw { from, force } => {
            if let Err(e) = command_throw(from.clone(), *force) {
                error(&e.to_string());
            }
        }
        Commands::External(args) => {
            if let Err(e) = handle_fallback_command(args) {
                error(&e.to_string());
            }
        }
    }
}
