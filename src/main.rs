mod commit;
mod init;
mod utils;

use anyhow::anyhow;
use clap::{Parser, Subcommand};
use commit::command_commit;
use duct::cmd;

use crate::{init::command_init, utils::error, utils::hint};

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
        /// Optional commit message
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

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    if let Err(e) = check_jj_installed() {
        error(&e.to_string());
        hint("https://jj-vcs.github.io/jj/latest/install-and-setup/");
    }
    match &cli.command {
        Commands::Init => {
            command_init()?;
        }
        Commands::Commit { message } => {
            command_commit(message.clone())?;
        }
        Commands::Push => {
            println!("Changes pushed.");
        }
        Commands::Pull => {
            println!("Changes pulled.");
        }
    }
    Ok(())
}
