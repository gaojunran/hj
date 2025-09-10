use duct::cmd;
use regex::Regex;

use crate::config::AppConfig;

pub(crate) fn command_clone(
    config: &AppConfig,
    source: &str,
    destination: Option<&str>,
    colocate: bool,
    fork: bool,
) -> anyhow::Result<()> {
    let (mut url, mut user, _) = build_info(
        &config.clone.default_host,
        config.clone.default_user.as_deref(),
        source,
    )
    .ok_or(anyhow::anyhow!("Invalid URL or fullname"))?;
    if fork && let Some(forker) = &config.clone.default_user {
        cmd!("gh", "repo", "fork", &url).run()?;
        url = url.replacen(&(user.clone() + "/"), &(forker.clone() + "/"), 1);
        user = forker.to_string(); // unused now
    }
    let mut args = vec!["git", "clone", &url];
    if let Some(destination) = destination {
        args.push(destination);
    }
    if colocate || config.always_colocate {
        args.push("--colocate");
    }
    cmd("jj", args).run()?;
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
