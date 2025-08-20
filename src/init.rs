use std::{fs, path::Path};

use anyhow::{Context, bail};
use duct::cmd;
use regex::Regex;

use crate::{config::AppConfig, utils::step};

pub(crate) fn command_init(
    config: &AppConfig,
    github: bool,
    private: bool,
    colocate: bool,
) -> anyhow::Result<()> {
    let mut already_init = false;
    if colocate && Path::new(".jj").exists() && !Path::new(".git").exists() {
        step("Converting into colocated repository...");
        convert_into_colocated()?;
        already_init = true;
    } else {
        step("Initializing jj repository...");
        let args = if colocate || config.always_colocate {
            vec!["git", "init", "--colocate"]
        } else {
            vec!["git", "init"]
        };
        if let Err(e) = cmd("jj", args).run() {
            if github {
                already_init = true;
            } else {
                anyhow::bail!("Failed to initialize jj repository: {e}");
            }
        };
    }

    if !already_init {
        let default_branch = config.init_config.default_branch.clone();
        step(format!("Setting default branch to `{default_branch}`...").as_str());
        cmd!("jj", "bookmark", "set", "-r", "@", default_branch).read()?;
    }

    // TODO: add a flag --no-github
    if config.init_config.create_github_repo || github {
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

fn convert_into_colocated() -> anyhow::Result<()> {
    // 1) # Ignore the .jj directory in Git
    // echo '/*' > .jj/.gitignore
    fs::write(".jj/.gitignore", "/*\n").context("write .jj/.gitignore")?;

    // 2) # Move the Git repo
    // mv .jj/repo/store/git .git
    let src_git = Path::new(".jj/repo/store/git");
    let dst_git = Path::new(".git");
    if !src_git.exists() {
        bail!("source Git dir not found: {}", src_git.display());
    }
    if dst_git.exists() {
        bail!(".git already exists; abort to avoid clobbering");
    }
    fs::create_dir_all(".jj/repo/store").context("ensure .jj/repo/store")?;
    fs::rename(src_git, dst_git)
        .with_context(|| format!("rename {} -> {}", src_git.display(), dst_git.display()))?;

    // 3) # Tell jj where to find it (do not use on Windows! See below.)
    // echo -n '../../../.git' > .jj/repo/store/git_target
    fs::write(".jj/repo/store/git_target", b"../../../.git")
        .context("write .jj/repo/store/git_target")?;

    // 4) # Make the Git repository non-bare and set HEAD
    // git config --unset core.bare
    let _ = cmd!("git", "config", "--unset", "core.bare").run();

    // 5) # Convince jj to update .git/HEAD to point to the working-copy commit's parent
    // jj new && jj undo
    cmd!("jj", "new").run().context("jj new failed")?;
    cmd!("jj", "undo").run().context("jj undo failed")?;

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
