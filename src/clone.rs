use duct::cmd;
use regex::Regex;

use crate::config::AppConfig;

pub(crate) fn command_clone(config: &AppConfig, url_or_fullname: &str) -> anyhow::Result<()> {
    let url = build_url(&config.default_host, url_or_fullname)
        .ok_or(anyhow::anyhow!("Invalid URL or fullname"))?;
    cmd!("jj", "git", "clone", &url).run()?;
    Ok(())
}

fn build_url(host: &str, url_or_fullname: &str) -> Option<String> {
    let re_url = Regex::new(r"https://[^\s]*?\.git").unwrap();
    let re_fullname = Regex::new(r"^[^/]+/[^/]+$").unwrap();
    if let Some(cap) = re_url.find(url_or_fullname) {
        Some(cap.as_str().to_string())
    } else {
        re_fullname
            .find(url_or_fullname)
            .map(|cap| format!("https://{}/{}.git", host, cap.as_str()))
        // optional return
    }
}
