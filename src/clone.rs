use duct::cmd;
use regex::Regex;
use std::env;

use crate::{config::AppConfig, utils::step};

fn fork_repo(owner: &str, repo: &str) -> anyhow::Result<()> {
    // Get GitHub token from environment variable
    let token = env::var("GITHUB_TOKEN")
        .or_else(|_| env::var("GH_TOKEN"))
        .map_err(|_| anyhow::anyhow!("GITHUB_TOKEN or GH_TOKEN environment variable not set"))?;

    let client = reqwest::blocking::Client::new();
    let url = format!("https://api.github.com/repos/{}/{}/forks", owner, repo);

    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "hj-vcs")
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .send()?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response
            .text()
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(anyhow::anyhow!(
            "Failed to fork repository: {} - {}",
            status,
            error_text
        ));
    }

    step(&format!("Forked {}/{}", owner, repo));
    Ok(())
}

pub(crate) fn command_clone(
    config: &AppConfig,
    source: &str,
    destination: Option<&str>,
    colocate: bool,
    fork: bool,
) -> anyhow::Result<()> {
    let (mut url, user, repo) = build_info(
        &config.clone.default_host,
        config.clone.default_user.as_deref(),
        source,
    )
    .ok_or(anyhow::anyhow!("Invalid URL or fullname"))?;

    let original_url = url.clone();
    let is_forked = if fork {
        let forker = config
            .clone
            .default_user
            .as_ref()
            .ok_or(anyhow::anyhow!("default_user is required for forking"))?;

        // Fork via GitHub API
        fork_repo(&user, &repo)?;

        // Update URL to point to forked repo
        url = url.replacen(&(user.clone() + "/"), &(forker.clone() + "/"), 1);
        true
    } else {
        false
    };

    let mut args = vec!["git", "clone", &url];
    if let Some(destination) = destination {
        args.push(destination);
    }
    if colocate || config.always_colocate {
        args.push("--colocate");
    }
    cmd("jj", args).run()?;

    // Add upstream remote if we forked
    if is_forked {
        let clone_dir = destination.unwrap_or(&repo);
        cmd!("jj", "git", "remote", "add", "upstream", &original_url)
            .dir(clone_dir)
            .run()?;
        step(&format!("Added upstream remote: {}", original_url));
    }

    Ok(())
}

pub(crate) fn build_info(
    default_host: &str,
    default_user: Option<&str>,
    url_or_fullname: &str,
) -> Option<(String, String, String)> {
    let re_url = Regex::new(r"^https://([^/]+)/([^/]+)/([^.]+)\.git$").unwrap();
    let re_fullname = Regex::new(r"^([^/]+)/([^/]+)$").unwrap();
    let re_repo = Regex::new(r"^[^/]+$").unwrap();

    if let Some(caps) = re_url.captures(url_or_fullname) {
        // let _host = caps.get(1).unwrap().as_str();
        let user = caps.get(2).unwrap().as_str();
        let repo = caps.get(3).unwrap().as_str();
        Some((
            url_or_fullname.to_string(),
            user.to_string(),
            repo.to_string(),
        ))
    } else if let Some(caps) = re_fullname.captures(url_or_fullname) {
        // user/repo
        let user = caps.get(1).unwrap().as_str();
        let repo = caps.get(2).unwrap().as_str();
        let url = format!("https://{}/{}/{}.git", default_host, user, repo);
        Some((url, user.to_string(), repo.to_string()))
    } else if re_repo.is_match(url_or_fullname) {
        // repo only
        default_user.map(|user| {
            let repo = url_or_fullname;
            let url = format!("https://{}/{}/{}.git", default_host, user, repo);
            (url, user.to_string(), repo.to_string())
        })
    } else {
        None
    }
}
