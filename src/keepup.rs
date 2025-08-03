use duct::cmd;

use crate::config::AppConfig;

pub(crate) fn command_keepup(config: &AppConfig, branch: &Vec<String>) -> anyhow::Result<()> {
    if branch.is_empty() {
        cmd!(
            "jj",
            "bookmark",
            "move",
            "--from",
            "heads(::@ & bookmarks())",
            "--to",
            "heads(::@ & mutable() & ~description(exact:\"\") & (~empty() | merges()))"
        )
        .run()?;
        // from https://github.com/jj-vcs/jj/discussions/5568
        // and https://github.com/jj-vcs/jj/discussions/5568#discussioncomment-13007551
    } else {
        for bookmark in branch {
            cmd!("jj", "bookmark", "set", "-r", "@-", bookmark).run()?;
        }
    }
    Ok(())
}
