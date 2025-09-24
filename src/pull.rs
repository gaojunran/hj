use duct::cmd;

use crate::{
    config::AppConfig,
    utils::{step, warning},
};

pub(crate) fn command_pull(config: &AppConfig, branch: Option<String>) -> anyhow::Result<()> {
    step("Fetching changes from the remote...");
    cmd!("jj", "git", "fetch").run()?;
    if let Some(branch) = branch {
        step(format!("Rebasing on `{branch}`...").as_str());
        cmd!("jj", "rebase", "-d", &branch).run()?;
    } else {
        warning("No branch specified in `hj pull`. You may need to rebase by yourself.");
    }
    Ok(())
}
