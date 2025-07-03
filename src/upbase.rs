use duct::cmd;

use crate::{
    config::AppConfig,
    utils::{step, warning},
};

pub(crate) fn command_upbase(
    config: &AppConfig,
    branch: &Vec<String>,
    fetch: bool,
) -> anyhow::Result<()> {
    // we only need fetch, not pull,
    // because we assume the trunk() has no local changes
    if fetch || config.upbase_config.fetch {
        step("Fetching remote changes...");
        cmd!("jj", "git", "fetch").run()?;
    }
    if branch.is_empty() {
        step("Rebasing all branches onto the trunk...");
        cmd!(
            "jj",
            "rebase",
            "-d",
            "trunk()",
            "-s",
            "all:(::trunk())+ & mutable()"
        )
        .run()?; // from https://github.com/jj-vcs/jj/discussions/4974
    } else {
        for b in branch {
            step(format!("Rebase branch {b} onto the trunk...").as_str());
            cmd!("jj", "rebase", "-d", "trunk()", "-s", &b).run()?;
        }
    }
    Ok(())
}
