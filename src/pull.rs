use duct::cmd;

use crate::{config::AppConfig, utils::warning};

pub(crate) fn command_pull(config: &AppConfig, branch: Option<String>) -> anyhow::Result<()> {
    cmd!("jj", "git", "fetch").run()?;
    if let Some(branch) = branch {
        cmd!("jj", "rebase", "-d", &branch).run()?;
    } else {
        warning("No branch specified in `hj pull`. You may need to rebase by yourself.");
    }
    Ok(())
}
