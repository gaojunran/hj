use std::iter::FromIterator;

use crate::{
    config::AppConfig,
    utils::{step, warning},
};

/// Fetch bookmarks from origin and run `jj git fetch`.
///
/// Behavior:
/// - If `branches` is non-empty:
///     1. Run `jj bookmark track <branch>@origin ...`
/// - Always run:
///     2. `jj git fetch` (if branches non-empty, append `--bookmark <branch>` for each branch)
///
/// This function intentionally does not fail the whole operation if tracking bookmarks fails;
/// it will warn and continue to attempt the fetch.
pub(crate) fn command_fetch(_config: &AppConfig, branches: Vec<String>) -> anyhow::Result<()> {
    // If there are branches, run `jj bookmark track <b>@origin ...`
    if !branches.is_empty() {
        step("Tracking bookmarks from origin...");
        // build arguments: ["bookmark", "track", "b1@origin", "b2@origin", ...]
        let mut track_args: Vec<String> = Vec::with_capacity(2 + branches.len());
        track_args.push("bookmark".to_string());
        track_args.push("track".to_string());
        for b in &branches {
            track_args.push(format!("{}@origin", b));
        }
        // Convert to &str slice
        let track_args_ref: Vec<&str> = track_args.iter().map(|s| s.as_str()).collect();
        if let Err(e) = duct::cmd("jj", &track_args_ref).run() {
            warning(&format!(
                "Failed to run `jj bookmark track ...`: {}. Continuing with fetch...",
                e
            ));
        }
    }

    // Always run fetch. If branches provided, append --bookmark <branch> for each.
    step("Fetching changes from the remote...");
    // build ["git", "fetch", "--bookmark", "b1", "--bookmark", "b2", ...] or ["git", "fetch"]
    let mut fetch_args: Vec<String> = Vec::with_capacity(2 + branches.len() * 2);
    fetch_args.push("git".to_string());
    fetch_args.push("fetch".to_string());
    if !branches.is_empty() {
        for b in &branches {
            fetch_args.push("--bookmark".to_string());
            fetch_args.push(b.clone());
        }
    }
    let fetch_args_ref: Vec<&str> = fetch_args.iter().map(|s| s.as_str()).collect();
    // Propagate error if fetch fails.
    duct::cmd("jj", &fetch_args_ref).run()?;

    Ok(())
}
