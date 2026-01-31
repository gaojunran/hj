use duct::cmd;

use crate::{
    config::AppConfig,
    hook::run_hook,
    push::push_with_defaults,
    tools::check_immutable,
    utils::{build_jj_args, step, warning},
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
    if push {
        push_with_defaults(config)?;
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
    if let Some(pre_commit) = &config.hooks.pre_commit
        && !no_pre_hook
    {
        run_hook(config, pre_commit.clone(), "pre-commit", None)?;
    }

    let into = into.unwrap_or_else(|| "@-".to_string());
    let force = check_immutable(&into)?;
    if force {
        warning("You are modifying an immutable revset!");
    }

    let args = build_jj_args(&[
        "squash",
        "--interactive",
        "--from",
        "@",
        "--into",
        &into,
        if force { "--ignore-immutable" } else { "" },
    ]);
    cmd("jj", &args).run()?;

    if let Some(post_commit) = &config.hooks.post_commit
        && !no_post_hook
    {
        run_hook(config, post_commit.clone(), "post-commit", None)?;
    }
    if push {
        push_with_defaults(config)?;
    }
    Ok(())
}

pub(crate) fn command_reset(
    config: &AppConfig,
    from: Option<String>,
    push: bool,
) -> anyhow::Result<()> {
    let from = from.unwrap_or_else(|| "@-".to_string());
    let force = check_immutable(&from)?;
    if force {
        warning("You are modifying an immutable revset!");
    }

    let args = build_jj_args(&[
        "squash",
        "--interactive",
        "--from",
        &from,
        "--into",
        "@",
        if force { "--ignore-immutable" } else { "" },
    ]);
    cmd("jj", &args).run()?;

    if push {
        push_with_defaults(config)?;
    }
    Ok(())
}

pub(crate) fn command_throw(from: Option<String>, force: bool) -> anyhow::Result<()> {
    let from_ref = from.as_deref().unwrap_or("@");
    let args = build_jj_args(&[
        "restore",
        "--interactive",
        "--changes-in",
        from_ref,
        if force { "--ignore-immutable" } else { "" },
    ]);
    cmd("jj", &args).run()?;
    Ok(())
}
