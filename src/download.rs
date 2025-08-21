use anyhow::{Context, Result};
use flate2::read::GzDecoder;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue, USER_AGENT};
use serde::Deserialize;
use std::env;
use std::fs::{File, create_dir_all};
use std::io::Write;
use std::path::{Path, PathBuf};
use tar::Archive;

use crate::config::AppConfig;

#[derive(Debug, Deserialize)]
struct GithubEntry {
    name: String,
    path: String,
    #[serde(rename = "type")]
    kind: String,
    download_url: Option<String>,
    url: String,
}

// TODO: support gitlab
// TODO: support branches and tags
pub(crate) fn command_download(
    config: &AppConfig,
    source: &str,
    destination: Option<&str>,
    entries: Vec<String>,
) -> Result<()> {
    let (owner, repo) = parse_repo(source).context("Invalid URL or fullname")?;
    let base_path = destination.unwrap_or(&repo);

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("rust-client"));

    if let Ok(token) = std::env::var("GITHUB_TOKEN") {
        let value = format!("token {token}");
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&value).expect("Invalid GITHUB_TOKEN"),
        );
    }

    let client = Client::builder().default_headers(headers).build()?;

    if entries.is_empty() {
        let url = build_url_whole(&owner, &repo).context("Could not build download URL")?;
        download_whole(&client, &url, base_path)?;
    } else {
        create_dir_all(base_path)?;

        for entry in entries {
            let url = format!("https://api.github.com/repos/{owner}/{repo}/contents/{entry}");
            let out_path = Path::new(base_path).join(&entry);
            download_recursive(&client, &url, &out_path)?;
        }
    }

    Ok(())
}

// code from https://github.com/alok8bb/cloneit
fn download_recursive(client: &Client, url: &str, out_path: &Path) -> Result<()> {
    let res = client
        .get(url)
        // .header("User-Agent", "rust-client")
        .send()
        .context("Request failed")?
        .error_for_status()
        .context("GitHub returned error status")?;

    let text = res.text()?;

    if text.trim_start().starts_with('[') {
        let entries: Vec<GithubEntry> = serde_json::from_str(&text)?;
        for entry in entries {
            if entry.kind == "dir" {
                let sub_path = out_path.join(&entry.name);
                create_dir_all(&sub_path)?;
                download_recursive(client, &entry.url, &sub_path)?;
            } else if entry.kind == "file"
                && let Some(dl_url) = entry.download_url
            {
                let content = client
                    .get(&dl_url)
                    // .header("User-Agent", "rust-client")
                    .send()?
                    .error_for_status()?;
                let bytes = content.bytes()?;
                let file_path = out_path.join(&entry.name);
                if let Some(parent) = file_path.parent() {
                    create_dir_all(parent)?;
                }
                let mut f = File::create(file_path)?;
                f.write_all(&bytes)?;
                println!("+ {}", entry.path);
            }
        }
    } else {
        let entry: GithubEntry = serde_json::from_str(&text)?;
        if entry.kind == "file"
            && let Some(dl_url) = entry.download_url
        {
            let content = client
                .get(&dl_url)
                // .header("User-Agent", "rust-client")
                .send()?
                .error_for_status()?;
            let bytes = content.bytes()?;
            if let Some(parent) = out_path.parent() {
                create_dir_all(parent)?;
            }
            let mut f = File::create(out_path)?;
            f.write_all(&bytes)?;
            println!("+ {}", entry.path);
        }
    }
    Ok(())
}

fn parse_repo(input: &str) -> Option<(String, String)> {
    if let Some(stripped) = input
        .strip_prefix("http://")
        .or_else(|| input.strip_prefix("https://"))
    {
        let parts: Vec<&str> = stripped.split('/').collect();
        if parts.len() >= 3 && parts[0].ends_with(".com") {
            return Some((
                parts[1].to_string(),
                parts[2].to_string().replace(".git", ""),
            ));
        }
    } else {
        let parts: Vec<&str> = input.split('/').collect();
        if parts.len() == 2 {
            return Some((parts[0].to_string(), parts[1].to_string()));
        }
    }

    None
}

fn build_url_whole(owner: &str, repo: &str) -> Option<String> {
    Some(format!(
        "https://github.com/{owner}/{repo}/archive/HEAD.tar.gz"
    ))
}

fn download_whole(client: &Client, url: &String, destination: &str) -> Result<()> {
    // let client = reqwest::blocking::Client::builder().build()?;
    let response = client.get(url).send()?;
    match response.status() {
        reqwest::StatusCode::OK => (),
        reqwest::StatusCode::UNAUTHORIZED => {
            Err(anyhow::anyhow!("Could not find repository."))?;
        }
        s => Err(anyhow::anyhow!("Received response status: {:?}", s))?,
    };

    let total_size = response.content_length();

    let pb = match total_size {
        Some(x) => {
            let p = ProgressBar::new(x);
            p.set_style(ProgressStyle::default_bar()
                     .template("> {wide_msg}\n{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
                     .progress_chars("#>-"));
            p
        }
        None => ProgressBar::new_spinner(),
    };

    println!("> Downloading from {url}");

    let reader = pb.wrap_read(response);
    let tar = GzDecoder::new(reader);
    let mut archive = Archive::new(tar);

    let dest = env::current_dir()?.join(destination);

    archive
        .entries()?
        .filter_map(|e| e.ok())
        .map(|mut entry| -> Result<PathBuf, anyhow::Error> {
            let path = entry.path()?;
            let path = path
                .strip_prefix(path.components().next().unwrap())?
                .to_owned();
            entry.unpack(dest.join(&path))?;
            Ok(path)
        })
        .filter_map(|e| e.ok())
        .for_each(|x| pb.set_message(format!("{}", x.display())));

    pb.finish_with_message("Done...");
    Ok(())
}
