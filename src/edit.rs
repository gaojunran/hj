use duct::cmd;

use crate::config::AppConfig;

pub(crate) fn command_edit(
    config: &AppConfig,
    rest: Vec<String>,
    mine: bool,
) -> anyhow::Result<()> {
    if mine {
        #[cfg(not(unix))]
        {
            return Err(anyhow::anyhow!("--mine flag is not supported on Windows"));
        }

        #[cfg(unix)]
        {
            // If --mine flag is set, pick from log_mine
            let rev = crate::log::pick_from_log_mine()?;
            cmd!("jj", "edit", &rev).run()?;
        }
    } else if rest.is_empty() {
        // No arguments provided, use default @
        cmd!("jj", "edit", "@").run()?;
    } else {
        // Pass all arguments (revisions and flags) to jj edit
        let mut args = vec!["edit"];
        args.extend(rest.iter().map(|s| s.as_str()));
        cmd("jj", &args).run()?;
    }
    Ok(())
}
