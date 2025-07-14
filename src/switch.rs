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
    cmd!("jj", "new", &branch).run()?;
    if git {
        check_git_installed()?;
        cmd!("git", "checkout", &branch).run()?;
    }
    Ok(())
}
