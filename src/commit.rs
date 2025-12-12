use duct::cmd;

use crate::{
    config::AppConfig,
    hook::run_hook,
    push::command_push,
    tools::check_immutable,
    utils::{step, warning},
};

pub(crate) fn command_commit(
    config: &AppConfig,
    message: Option<String>,
    push: bool,
    abandon: bool,
    no_pre_hook: bool,
    no_post_hook: bool,
) -> anyhow::Result<()> {
    if let Some(pre_commit) = &config.hooks.pre_commit
        && !no_pre_hook
    {
        run_hook(config, pre_commit.clone(), "pre-commit", None)?;
    }
    if let Some(msg) = message {
        cmd!("jj", "commit", "--interactive", "--message", msg).run()?;
    } else {
        cmd!("jj", "commit", "--interactive").run()?;
    }
    if let Some(post_commit) = &config.hooks.post_commit
        && !no_post_hook
    {
        run_hook(config, post_commit.clone(), "post-commit", None)?;
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
            false,
            false,
        )?;
    }
    if abandon {
        step("Abandoning uncommitted changes...");
        cmd!("jj", "abandon", "@").run()?;
    }
    Ok(())
}

pub(crate) fn command_amend(
    config: &AppConfig,
    into: Option<String>,
    push: bool,
    no_pre_hook: bool,
    no_post_hook: bool,
) -> anyhow::Result<()> {
    let mut force = false;

    if let Some(pre_commit) = &config.hooks.pre_commit
        && !no_pre_hook
    {
        run_hook(config, pre_commit.clone(), "pre-commit", None)?;
    }

    let into = into.unwrap_or("@-".to_string());

    if check_immutable(&into)? {
        warning("You are modifying an immutable revset!");
        force = true;
    }

    let args: Vec<&str> = vec![
        "squash",
        "--interactive",
        "--from",
        "@",
        "--into",
        &into,
        if force { "--ignore-immutable" } else { "" },
    ]
    .into_iter()
    .filter(|s| !s.is_empty())
    .collect();
    cmd("jj", &args).run()?;

    if let Some(post_commit) = &config.hooks.post_commit
        && !no_post_hook
    {
        run_hook(config, post_commit.clone(), "post-commit", None)?;
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
            false,
            false,
        )?;
    }
    Ok(())
}

pub(crate) fn command_reset(
    config: &AppConfig,
    from: Option<String>,
    push: bool,
) -> anyhow::Result<()> {
    let mut force = false;

    let from = from.unwrap_or("@-".to_string());

    if check_immutable(&from)? {
        warning("You are modifying an immutable revset!");
        force = true;
    }

    let args: Vec<&str> = vec![
        "squash",
        "--interactive",
        "--from",
        &from,
        "--into",
        "@",
        if force { "--ignore-immutable" } else { "" },
    ]
    .into_iter()
    .filter(|s| !s.is_empty())
    .collect();

    cmd("jj", &args).run()?;
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
            false,
            false,
        )?;
    }
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
