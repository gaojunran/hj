use std::path::Path;

use duct::cmd;

use crate::{config::AppConfig, utils::step};

pub(crate) fn check_just_installed() -> bool {
    cmd!("just", "--version").run().is_ok()
}

pub(crate) fn run_hook(
    config: &AppConfig,
    script: String,
    hook_name: &str,
) -> anyhow::Result<bool> {
    if config.hooks.use_just && !check_just_installed() {
        step("Install just to run hooks");
        // TODO: provide installation instructions
    }

    // Run git hook
    if Path::new(&format!(".git/hooks/{hook_name}")).exists()
        && !config
            .hooks
            .ignore_git_hooks
            .contains(&hook_name.to_string())
    {
        step(&format!("Running {hook_name} hook"));
        if let Err(e) = cmd!("git", "hook", "run", hook_name).run()
            && hook_name.starts_with("pre-")
        {
            anyhow::bail!(
                "Git {} hook failed: {}. Aborting.",
                hook_name,
                e.to_string()
            )
        }
    }

    // Run hj hook
    let (program, args): (&str, Vec<&str>) = script
        .split_once(' ')
        .map(|(p, a)| (p, a.trim().split(' ').collect()))
        .unwrap_or((&script, Vec::new()));
    if let Err(e) = cmd(program, &args).run()
        && hook_name.starts_with("pre-")
    {
        anyhow::bail!("HJ {} hook failed: {}. Aborting.", hook_name, e.to_string());
    }
    Ok(true)
}
