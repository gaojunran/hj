use duct::cmd;

#[cfg(unix)]
use skim::prelude::*;
#[cfg(unix)]
use std::io::Cursor;

use crate::config::AppConfig;

pub(crate) fn command_log_all(config: &AppConfig) -> anyhow::Result<()> {
    cmd!("jj", "log", "-r", "all()").run()?;
    Ok(())
}

pub(crate) fn command_log_wip(config: &AppConfig, patch: bool) -> anyhow::Result<()> {
    // all heads on branches which is wip
    let revset = "heads(:: ~ description(exact:''))..";
    if patch {
        cmd!("jj", "log", "-r", revset, "--patch", "--summary").run()?;
    } else {
        cmd!("jj", "log", "-r", revset, "--summary").run()?;
    }
    Ok(())
}

pub(crate) fn command_log_mine(
    config: &AppConfig,
    patch: bool,
    summary: bool,
) -> anyhow::Result<()> {
    // all heads on branches which is mine
    let revset = "mine() & ~description(exact:'') & bookmarks()";
    let template = "bookmarks ++ ' | ' ++ change_id.shortest() ++ '/' ++ commit_id.shortest() ++ ' | ' ++ description.first_line() ++ '\n'";
    let mut command = vec![
        "log",
        "-r",
        revset,
        "-T",
        template,
        "--no-pager",
        "--no-graph",
    ];
    if patch {
        command.push("--patch");
    }
    if summary {
        command.push("--summary");
    }
    cmd("jj", &command).run()?;
    Ok(())
}

#[cfg(unix)]
pub(crate) fn pick_from_log_mine() -> anyhow::Result<String> {
    // Get the output from command_log_mine
    let revset = "mine() & ~description(exact:'') & bookmarks()";
    let template = "bookmarks ++ ' | ' ++ change_id.shortest() ++ '/' ++ commit_id.shortest() ++ ' | ' ++ description.first_line() ++ '\n'";
    let output = cmd!(
        "jj",
        "log",
        "-r",
        revset,
        "-T",
        template,
        "--no-pager",
        "--no-graph"
    )
    .read()
    .map_err(|e| anyhow::anyhow!("Failed to get log output: {}", e))?;

    if output.trim().is_empty() {
        return Err(anyhow::anyhow!("No commits found"));
    }

    // Setup skim options for single selection
    let options = SkimOptionsBuilder::default()
        .height("50%".to_string())
        .multi(false)
        .prompt("Select a commit: ".to_string())
        .build()
        .map_err(|e| anyhow::anyhow!("Failed to build skim options: {}", e))?;

    // Create item reader
    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(output));

    // Run skim and get selected item
    let skim_output = Skim::run_with(&options, Some(items));

    // Check if user cancelled (None means user pressed Ctrl+C or ESC)
    let skim_output = match skim_output {
        Some(out) => out,
        None => return Err(anyhow::anyhow!("Selection cancelled")),
    };

    // Check if user actually accepted a selection (Enter key)
    if skim_output.is_abort {
        return Err(anyhow::anyhow!("Selection aborted"));
    }

    let selected_items = skim_output.selected_items;
    if selected_items.is_empty() {
        return Err(anyhow::anyhow!("No commit selected"));
    }

    // Extract the shortest commit_id from the selected line
    // Format: "bookmarks | change_id/commit_id | description"
    let selected_line = selected_items[0].output().to_string();

    // Parse the line to extract commit_id (after the '/' and before the next '|')
    let parts: Vec<&str> = selected_line.split('|').collect();
    if parts.len() < 2 {
        return Err(anyhow::anyhow!("Invalid log format"));
    }

    let id_part = parts[1].trim();
    let commit_id = id_part
        .split('/')
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("Failed to extract commit_id"))?
        .trim();

    Ok(commit_id.to_string())
}

#[cfg(not(unix))]
pub(crate) fn pick_from_log_mine() -> anyhow::Result<String> {
    Err(anyhow::anyhow!(
        "Interactive selection is not supported on Windows"
    ))
}
