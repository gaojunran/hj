use duct::cmd;

use crate::config::AppConfig;

pub(crate) fn command_keepup(config: &AppConfig, branch: &Vec<String>) -> anyhow::Result<()> {
    let target = "heads(::@ & mutable() & ~description(exact:\"\") & (~empty() | merges()))";
    if branch.is_empty() {
        let source = if config.keepup.avoid_trunk {
            "heads(::@ & (bookmarks() ~ trunk()))"
        } else {
            "heads(::@ & bookmarks())"
        };
        cmd!("jj", "bookmark", "move", "--from", source, "--to", target).run()?;
        // from https://github.com/jj-vcs/jj/discussions/5568
        // and https://github.com/jj-vcs/jj/discussions/5568#discussioncomment-13007551
    } else {
        for bookmark in branch {
            cmd!("jj", "bookmark", "set", "-r", target, bookmark).run()?;
        }
    }
    Ok(())
}
