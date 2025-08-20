use duct::cmd;
use regex::Regex;

use crate::config::AppConfig;

pub(crate) fn command_clone(
    config: &AppConfig,
    source: &str,
    destination: Option<&str>,
    colocate: bool,
) -> anyhow::Result<()> {
    let url = build_url(
        &config.clone.default_host,
        config.clone.default_user.as_deref(),
        source,
    )
    .ok_or(anyhow::anyhow!("Invalid URL or fullname"))?;
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

fn build_url(
    default_host: &str,
    default_user: Option<&str>,
    url_or_fullname: &str,
) -> Option<String> {
    let re_url = Regex::new(r"^https://[^\s]*?\.git$").unwrap();
    let re_fullname = Regex::new(r"^[^/]+/[^/]+$").unwrap();
    let re_repo = Regex::new(r"^[^/]+$").unwrap();

    if re_url.is_match(url_or_fullname) {
        Some(url_or_fullname.to_string())
    } else if re_fullname.is_match(url_or_fullname) {
        // user/repo
        Some(format!("https://{}/{}.git", default_host, url_or_fullname))
    } else if re_repo.is_match(url_or_fullname) {
        // pass repo only, needs `default_user`
        default_user
            .map(|user| format!("https://{}/{}/{}.git", default_host, user, url_or_fullname))
    } else {
        None
    }
}
