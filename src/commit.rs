use duct::cmd;

use crate::{config::AppConfig, push::command_push, utils::step};

pub(crate) fn command_commit(
    config: &AppConfig,
    message: Option<String>,
    push: bool,
    abandon: bool,
) -> anyhow::Result<()> {
    if let Some(msg) = message {
        cmd!("jj", "commit", "--interactive", "--message", msg).run()?;
    } else {
        cmd!("jj", "commit", "--interactive").run()?;
    }
    // TODO: should we give more options here?
    if push {
        // step("Pushing changes...");
        command_push(
            config,
            &Vec::new(), // auto select what to push
            &Vec::new(), // no changes should be given
            config.push.still,
            config.push.pull,
            config.push.upbase,
        )?;
    }
    if abandon {
        step("Abandoning uncommitted changes...");
        cmd!("jj", "abandon", "@").run()?;
    }
    Ok(())
}

pub(crate) fn command_amend(into: Option<String>, force: bool) -> anyhow::Result<()> {
    let args: Vec<&str> = vec![
        "squash",
        "--interactive",
        "--from",
        "@",
        "--into",
        into.as_deref().unwrap_or("@-"),
        if force { "--ignore-immutable" } else { "" },
    ]
    .into_iter()
    .filter(|s| !s.is_empty())
    .collect();
    cmd("jj", &args).run()?;
    Ok(())
}

pub(crate) fn command_reset(from: Option<String>, force: bool) -> anyhow::Result<()> {
    let args: Vec<&str> = vec![
        "squash",
        "--interactive",
        "--from",
        from.as_deref().unwrap_or("@-"),
        "--into",
        "@",
        if force { "--ignore-immutable" } else { "" },
    ]
    .into_iter()
    .filter(|s| !s.is_empty())
    .collect();

    cmd("jj", &args).run()?;
    Ok(())
}

pub(crate) fn command_throw(from: Option<String>, force: bool) -> anyhow::Result<()> {
    let args: Vec<&str> = vec![
        "restore",
        "--interactive",
        "--changes-in",
        from.as_deref().unwrap_or("@"),
        if force { "--ignore-immutable" } else { "" },
    ]
    .into_iter()
    .filter(|s| !s.is_empty())
    .collect();

    cmd("jj", &args).run()?;
    Ok(())
}
