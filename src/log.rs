use duct::cmd;

use crate::config::AppConfig;

pub(crate) fn command_log_all(config: &AppConfig) -> anyhow::Result<()> {
    cmd!("jj", "log", "-r", "all()").run()?;
    Ok(())
}

pub(crate) fn command_log(config: &AppConfig) -> anyhow::Result<()> {}
