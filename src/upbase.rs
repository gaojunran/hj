use duct::cmd;

use crate::{
    config::AppConfig,
    utils::{step, warning},
};

pub(crate) fn command_upbase(config: &AppConfig, branch: Vec<String>) -> anyhow::Result<()> {
    if branch.is_empty() {
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
