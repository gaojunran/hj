use duct::cmd;

use crate::{
    config::AppConfig, keepup::command_keepup, pull::command_pull, upbase::command_upbase,
    utils::step,
};

pub(crate) fn command_push(
    config: &AppConfig,
    branch: &Vec<String>,
    change: &Vec<String>,
    keepup: bool,
    pull: bool,
    upbase: bool,
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

    let mut args = vec!["git", "push", "--allow-new"];
    if !branch.is_empty() {
        args.extend(branch.iter().flat_map(|i| ["--bookmark", i]));
    } else if !change.is_empty() {
        args.extend(change.iter().flat_map(|i| ["--change", i]));
    }

    // keepup if needed
    if config.push_config.keepup || keepup {
        step("Keepup bookmark(s)...");
        command_keepup(config, branch)?;
    }

    // push command
    step("Pushing changes...");
    cmd("jj", args).run()?;

    Ok(())
}
