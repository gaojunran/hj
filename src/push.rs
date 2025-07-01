use duct::cmd;

use crate::config::AppConfig;

pub(crate) fn command_push(
    config: &AppConfig,
    branch: &Vec<String>,
    keepup: bool,
) -> anyhow::Result<()> {
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
