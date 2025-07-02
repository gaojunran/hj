use duct::cmd;

use crate::{config::AppConfig, pull::command_pull};

pub(crate) fn command_push(
    config: &AppConfig,
    branch: &Vec<String>,
    keepup: bool,
    pull: bool,
) -> anyhow::Result<()> {
    if pull {
        if branch.len() != 1 {
            anyhow::bail!("`--pull` flag only works with pushing a single branch");
        }
        let branch = &branch[0];
        command_pull(config, Some(branch.to_string()))?;
    }
    let mut args = vec!["git", "push", "--allow-new"];
    if !branch.is_empty() {
        args.extend(branch.iter().flat_map(|i| ["--bookmark", i]));
        if config.push_config.keepup || keepup {
            for bookmark in branch {
                cmd!("jj", "bookmark", "set", "-r", "@-", bookmark).run()?;
            }
        }
    }

    // push command
    cmd("jj", args).run()?;

    Ok(())
}
