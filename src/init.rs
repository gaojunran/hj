use duct::cmd;

use crate::config::AppConfig;

pub(crate) fn command_init(config: &AppConfig) -> anyhow::Result<()> {
    cmd!("jj", "git", "init").run()?;
    cmd!(
        "jj",
        "bookmark",
        "set",
        "-r",
        "@",
        config.default_branch.clone()
    )
    .run()?;
    Ok(())
}
