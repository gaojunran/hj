use duct::cmd;

use crate::{check_git_installed, config::AppConfig, keepup::command_keepup, tools::log};

pub(crate) fn command_switch(
    config: &AppConfig,
    keepup_branch: &Vec<String>,
    dest_branch: String,
    // keepup: bool,  // currently deprecated
    git: bool,
) -> anyhow::Result<()> {
    command_keepup(config, keepup_branch)?;
    let aheads_rev = format!("({dest_branch}+) ~ bookmarks() & description(exact:\"\")");
    let aheads = log(&aheads_rev, "change_id ++ '\n'", true)?
        .trim()
        .lines()
        .count();
    if aheads == 0 {
        cmd!("jj", "new", &dest_branch).run()?;
    } else {
        let rev = format!("latest({aheads_rev})");
        cmd!("jj", "edit", &rev).run()?;
    }
    if git {
        check_git_installed()?;
        cmd!("git", "checkout", &dest_branch).run()?;
    }
    Ok(())
}
