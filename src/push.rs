use duct::cmd;

use crate::{
    config::AppConfig, hook::run_hook, keepup::command_keepup, pull::command_pull,
    upbase::command_upbase, utils::step,
};

pub(crate) fn command_push(
    config: &AppConfig,
    branch: &Vec<String>,
    change: &Vec<String>,
    still: bool, // do not keepup. often seen in stacked prs.
    pull: bool,
    upbase: bool,
    no_pre_hook: bool,
    no_post_hook: bool,
    with_tags: Option<&Vec<String>>,
    // None -> no tag; Zero length -> all tags; Some -> specified tags
) -> anyhow::Result<()> {
    if pull {
        if branch.len() != 1 {
            anyhow::bail!("`--pull` flag only works with pushing a single branch");
        }
        let branch = &branch[0];
        step(format!("Pulling {branch} before pushing...").as_str());
        command_pull(config, Some(branch.to_string()))?;
    }

    if upbase {
        step("Upbase before pushing...");
        command_upbase(config, branch, !pull)?; // if pull is true, then fetch is not needed
    }

    if let Some(pre_push) = &config.hooks.pre_push
        && !no_pre_hook
    {
        let stdin_input = if !branch.is_empty() {
            Some(branch.join(" "))
        } else {
            None
        };
        run_hook(config, pre_push.clone(), "pre-push", stdin_input)?;
    }

    let mut args = vec!["git", "push", "--allow-new"];
    if !branch.is_empty() {
        args.extend(branch.iter().flat_map(|i| ["--bookmark", i]));
    } else if !change.is_empty() {
        args.extend(change.iter().flat_map(|i| ["--change", i]));
    }

    // keepup if needed
    if !(config.push.still || still) && change.is_empty() {
        step("Keepup bookmarks...");
        command_keepup(config, branch)?;
    }

    // push command
    step("Pushing changes...");
    cmd("jj", args).run()?;

    // push tags with git command
    if let Some(with_tags) = with_tags {
        step("Pushing tags...");
        cmd(
            "git",
            vec!["push", "origin"]
                .into_iter()
                .chain(if with_tags.is_empty() {
                    vec!["--tags"]
                } else {
                    with_tags.iter().map(|s| s.as_str()).collect()
                })
                .collect::<Vec<&str>>(),
        )
        .run()?;
    }

    if let Some(post_push) = &config.hooks.post_push
        && !no_post_hook
    {
        let stdin_input = if !branch.is_empty() {
            Some(branch.join(" "))
        } else {
            None
        };
        run_hook(config, post_push.clone(), "post-push", stdin_input)?;
    }

    Ok(())
}
