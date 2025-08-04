use duct::cmd;

use crate::config::AppConfig;

pub(crate) fn command_log_all(config: &AppConfig) -> anyhow::Result<()> {
    cmd!("jj", "log", "-r", "all()").run()?;
    Ok(())
}

pub(crate) fn command_log_wip(config: &AppConfig, patch: bool) -> anyhow::Result<()> {
    // all heads on branches which is wip
    let revset = "heads(:: ~ description(exact:''))..";
    if patch {
        cmd!("jj", "log", "-r", revset, "--patch", "--summary").run()?;
    } else {
        cmd!("jj", "log", "-r", revset, "--summary").run()?;
    }
    Ok(())
}
