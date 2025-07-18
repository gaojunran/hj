use duct::cmd;

use crate::config::AppConfig;

pub(crate) fn command_logall(config: &AppConfig) -> anyhow::Result<()> {
    cmd!("jj", "log", "-r", "all()").run()?;
    Ok(())
}
