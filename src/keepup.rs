use duct::cmd;

use crate::config::AppConfig;

pub(crate) fn command_keepup(config: &AppConfig, branch: &Vec<String>) -> anyhow::Result<()> {
    if branch.is_empty() {
        cmd!(
            "jj",
            "bookmark",
            "move",
            "--from",
            "heads(::@- & bookmarks())",
            "--to",
            "@-"
        )
        .run()?; // from https://github.com/jj-vcs/jj/discussions/5568
    } else {
        for bookmark in branch {
            cmd!("jj", "bookmark", "set", "-r", "@-", bookmark).run()?;
        }
    }
    Ok(())
}
