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
        cmd!("jj", "desc", "-r", "@-", "--message", desc).run()?;
    }
    Ok(())
}

pub(crate) fn command_amend(into: Option<String>) -> anyhow::Result<()> {
    cmd!(
        "jj",
        "squash",
        "--interactive",
        "--from",
        "@",
        "--into",
        if let Some(into) = into {
            into
        } else {
            "@-".to_string()
        }
    )
    .run()?;
    Ok(())
}

pub(crate) fn command_reset(from: Option<String>) -> anyhow::Result<()> {
    cmd!(
        "jj",
        "squash",
        "--interactive",
        "--from",
        if let Some(from) = from {
            from
        } else {
            "@-".to_string()
        },
        "--into",
        "@"
    )
    .run()?;
    Ok(())
}
