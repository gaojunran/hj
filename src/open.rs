use duct::cmd;

use crate::{config::AppConfig, utils::warning};

pub(crate) fn command_open(config: &AppConfig, remote: Option<String>) -> anyhow::Result<()> {
    match remote {
        None => {
            let path = cmd!("jj", "root").read()?;
            if let Some(editor) = &config.open.editor {
                cmd!(editor, path).run()?;
                return Ok(());
            }
            match std::env::var("EDITOR") {
                Ok(editor) => {
                    cmd!(editor, path).run()?;
                }
                Err(_) => {
                    warning("No editor found. Please set one either in $EDITOR or in hj's config");
                }
            };
        }
        Some(remote) => {
            let output = cmd!("jj", "git", "remote", "list").read()?;
            let remote_line = output
                .lines()
                .find(|x| x.contains(&remote))
                .ok_or_else(|| anyhow::anyhow!("Remote not found"))?;
            let remote_url = remote_line.split_once(' ').unwrap().1;
            webbrowser::open(remote_url)?;
        }
    }
    Ok(())
}
