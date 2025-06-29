use dialoguer::Input;
use duct::cmd;

pub(crate) fn command_commit(message: Option<String>) -> anyhow::Result<()> {
    if let Some(msg) = message {
        cmd!("jj", "commit", "--interactive", "--message", msg).run()?;
    } else {
        cmd!(
            "jj",
            "commit",
            "--interactive",
            "--message",
            "[placeholder commit message by hj]"
        )
        .run()?;
        let desc = Input::<String>::new()
            .with_prompt("Enter commit message")
            .interact_text()?;
        cmd!("jj", "desc", "--message", desc).read()?;
    }
    Ok(())
}
