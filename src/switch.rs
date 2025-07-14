use std::fmt::format;

use duct::cmd;

use crate::{check_git_installed, config::AppConfig, keepup::command_keepup};

pub(crate) fn command_switch(
    config: &AppConfig,
    branch: String,
    keepup: bool,
    git: bool,
) -> anyhow::Result<()> {
    if config.switch_config.keepup || keepup {
        command_keepup(config, &vec![branch.clone()])?;
    }
    // if there are multiple commits on top of the branch bookmark, switch to the latest one
    let rev = format!("latest({branch}+)");
    cmd!("jj", "edit", &rev).run()?;
    if git {
        check_git_installed()?;
        cmd!("git", "checkout", &branch).run()?;
    }
    Ok(())
}
