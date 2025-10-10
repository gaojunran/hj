use duct::cmd;
use std::env;

use crate::{
    config::AppConfig,
    utils::{step, warning},
};

/// Check if upstream remote exists
fn has_upstream_remote() -> anyhow::Result<bool> {
    let output = cmd!("jj", "git", "remote", "list")
        .stderr_null()
        .stdout_capture()
        .run()?;

    let output_str = String::from_utf8_lossy(&output.stdout);
    Ok(output_str.lines().any(|line| line.starts_with("upstream ")))
}

/// Extract owner and repo from origin remote URL
fn get_origin_info() -> anyhow::Result<(String, String)> {
    let output = cmd!("jj", "git", "remote", "list")
        .stderr_null()
        .stdout_capture()
        .run()?;

    let output_str = String::from_utf8_lossy(&output.stdout);
    let re = regex::Regex::new(r"github\.com[:/]([^/]+)/([^/.]+)").unwrap();

    for line in output_str.lines() {
        if line.starts_with("origin ") {
            let url = line.trim_start_matches("origin ").trim();
            // Parse GitHub URL to extract owner/repo
            // Support formats: https://github.com/owner/repo.git or git@github.com:owner/repo.git
            if let Some(captures) = re.captures(url) {
                let owner = captures.get(1).unwrap().as_str().to_string();
                let repo = captures.get(2).unwrap().as_str().to_string();
                return Ok((owner, repo));
            }
        }
    }

    Err(anyhow::anyhow!(
        "Could not find origin remote or parse GitHub URL"
    ))
}

/// Sync fork with upstream using GitHub API
fn sync_fork_with_upstream(owner: &str, repo: &str) -> anyhow::Result<()> {
    step("Syncing fork with upstream via GitHub API...");

    // Get GitHub token from environment variable
    let token = env::var("GITHUB_TOKEN")
        .or_else(|_| env::var("GH_TOKEN"))
        .map_err(|_| anyhow::anyhow!("GITHUB_TOKEN or GH_TOKEN environment variable not set. Please set it to sync your fork."))?;

    let client = reqwest::blocking::Client::new();
    let url = format!(
        "https://api.github.com/repos/{}/{}/merge-upstream",
        owner, repo
    );

    let body = serde_json::json!({
        "branch": "main"
    });

    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "hj-vcs")
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .json(&body)
        .send()?;

    if response.status().is_success() {
        step("Successfully synced fork with upstream");
        Ok(())
    } else if response.status().as_u16() == 409 {
        // 409 means the branch is already up-to-date
        step("Fork is already up-to-date with upstream");
        Ok(())
    } else {
        let status = response.status();
        let error_text = response
            .text()
            .unwrap_or_else(|_| "Unknown error".to_string());
        warning(&format!(
            "Failed to sync fork with upstream: {} - {}. Continuing with fetch...",
            status, error_text
        ));
        // Don't fail the entire pull operation
        Ok(())
    }
}

pub(crate) fn command_pull(_config: &AppConfig, branch: Option<String>) -> anyhow::Result<()> {
    // Check if upstream remote exists, and if so, sync the fork first
    if has_upstream_remote()? {
        match get_origin_info() {
            Ok((owner, repo)) => {
                if let Err(e) = sync_fork_with_upstream(&owner, &repo) {
                    warning(&format!(
                        "Could not sync fork: {}. Continuing with fetch...",
                        e
                    ));
                }
            }
            Err(e) => {
                warning(&format!(
                    "Could not determine origin info: {}. Continuing with fetch...",
                    e
                ));
            }
        }
    }

    step("Fetching changes from the remote...");
    cmd!("jj", "git", "fetch").run()?;
    if let Some(branch) = branch {
        step(format!("Rebasing on `{branch}`...").as_str());
        cmd!("jj", "rebase", "-d", &branch).run()?;
    } else {
        warning("No branch specified in `hj pull`. You may need to rebase by yourself.");
    }
    Ok(())
}
