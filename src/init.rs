use duct::cmd;

pub(crate) fn command_init() -> anyhow::Result<()> {
    cmd!("jj", "git", "init").run()?;
    Ok(())
}
