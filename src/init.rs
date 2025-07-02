use duct::cmd;
use regex::Regex;

use crate::{config::AppConfig, utils::step};

pub(crate) fn command_init(config: &AppConfig, github: bool, private: bool) -> anyhow::Result<()> {
    step("Initializing jj repository...");
    cmd!("jj", "git", "init").run()?;
    let default_branch = config.init_config.default_branch.clone();
    step(format!("Setting default branch to `{default_branch}`...").as_str());
    cmd!("jj", "bookmark", "set", "-r", "@", default_branch).read()?;

    // if `create_github_repo` is true and `github` is false, what to do?
    if config.init_config.create_github_repo || (!config.init_config.create_github_repo && github) {
        step("Creating GitHub repository...");
        let dirname = std::env::current_dir()?
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
        // create github repo using gh
        let repo_create_output = cmd!(
            "gh",
            "repo",
            "create",
            &dirname,
            if private { "--private" } else { "--public" }
        )
        .read()?;

        let re = Regex::new(r"https://[^\s]*").unwrap();
        if let Some(cap) = re.find(&repo_create_output) {
            let remote_url = cap.as_str().to_string() + ".git";
            step(format!("Setting remote origin to `{remote_url}`...").as_str());
            cmd!(
                "jj",
                "git",
                "remote",
                "add",
                config.init_config.default_remote_name.clone(),
                remote_url
            )
            .read()?;
        } else {
            anyhow::bail!(
                "Failed to parse remote URL. Maybe the repository was not created successfully?"
            );
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::{BufRead, BufReader};

    use duct::cmd;

    #[test]
    fn test_duct_output() -> anyhow::Result<()> {
        let reader = cmd!("jj", "git", "init").stderr_to_stdout().reader()?;
        let lines = BufReader::new(reader).lines();
        println!("Output from `jj git init`:");
        for line in lines {
            println!("{}", line?);
        }
        // println!("{output}");
        Ok(())
    }
}
