use dialoguer::Input;
use duct::cmd;

use crate::{config::AppConfig, push::command_push, utils::step};

pub(crate) fn command_commit(
    config: &AppConfig,
    message: Option<String>,
    push: bool,
) -> anyhow::Result<()> {
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
    // TODO: should we give more options here?
    if push {
        // step("Pushing changes...");
        command_push(
            config,
            &Vec::new(), // auto select what to push
            &Vec::new(), // no changes should be given
            config.push_config.keepup,
            config.push_config.pull,
            config.push_config.upbase,
        )?;
    }
    Ok(())
}

pub(crate) fn command_amend(into: Option<String>, force: bool) -> anyhow::Result<()> {
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
        },
        if force { "--ignore-immutable" } else { "" }
    )
    .run()?;
    Ok(())
}

pub(crate) fn command_reset(from: Option<String>, force: bool) -> anyhow::Result<()> {
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
        "@",
        if force { "--ignore-immutable" } else { "" }
    )
    .run()?;
    Ok(())
}
