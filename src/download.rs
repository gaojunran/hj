use flate2::read::GzDecoder;
use std::{env, path::PathBuf};
// use futures::executor::block_on;
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;
use tar::Archive;

use crate::{config::AppConfig, utils::step};

pub(crate) fn command_download(
    config: &AppConfig,
    url_or_fullname: &str,
    name: Option<&str>,
) -> anyhow::Result<()> {
    let url =
        build_download_url(url_or_fullname).ok_or(anyhow::anyhow!("Invalid URL or fullname"))?;
    download(&url, name)?;
    Ok(())
}

fn build_download_url(url_or_fullname: &str) -> Option<String> {
    let re_url = Regex::new(r"https://[^/]+/[^/]+(\.git)?").unwrap();
    let re_fullname = Regex::new(r"^[^/]+/[^/]+$").unwrap();
    if let Some(cap) = re_url.find(url_or_fullname) {
        Some(cap.as_str().replace(".git", "") + "/archive/HEAD.tar.gz")
    } else {
        re_fullname
            .find(url_or_fullname)
            .map(|cap| format!("https://github.com/{}/archive/HEAD.tar.gz", cap.as_str()))
        // optional return
    }
}

/// code from https://github.com/psnszsn/degit-rs/blob/c7dbeb75131510a79400838e081b90665c654c80/src/lib.rs#L115-L180
fn download(url: &String, name: Option<&str>) -> anyhow::Result<()> {
    // println!("{url}");
    let client = reqwest::blocking::Client::builder()
        // .user_agent("python-requests/2.32.3")
        .build()?;
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

    step(format!("Downloading from {url}").as_str());

    let reader = pb.wrap_read(response);
    let tar = GzDecoder::new(reader);
    let mut archive = Archive::new(tar);

    let replaced = url.replace("/archive/HEAD.tar.gz", "");

    let dest = env::current_dir()?.join(if let Some(name) = name {
        name
    } else {
        replaced
            .split('/')
            .next_back()
            .ok_or(anyhow::anyhow!("Invalid URL"))?
    });

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
