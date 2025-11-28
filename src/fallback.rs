use anyhow::{Result, anyhow};
use duct::cmd;

pub struct FallbackCommand {
    name: &'static str,
    aliases: Vec<&'static str>,
    before_execute: Option<fn() -> Result<()>>,
    after_execute: Option<fn() -> Result<()>>,
    rewrite_args: Option<fn(Vec<String>) -> Result<Vec<String>>>,
}

impl FallbackCommand {
    /// Returns the canonical command name if the input matches this command or its aliases
    fn matches(&self, command: &str) -> Option<&'static str> {
        if self.name == command || self.aliases.iter().any(|alias| *alias == command) {
            Some(self.name)
        } else {
            None
        }
    }
}

fn get_fallback_commands() -> Vec<FallbackCommand> {
    vec![
        FallbackCommand {
            name: "abandon",
            aliases: vec!["ab"],
            before_execute: None,
            after_execute: None,
            rewrite_args: None,
        },
        FallbackCommand {
            name: "absorb",
            aliases: vec![],
            before_execute: None,
            after_execute: None,
            rewrite_args: None,
        },
        FallbackCommand {
            name: "backout",
            aliases: vec![],
            before_execute: None,
            after_execute: None,
            rewrite_args: None,
        },
        FallbackCommand {
            name: "bookmark",
            aliases: vec!["b"],
            before_execute: None,
            after_execute: None,
            rewrite_args: None,
        },
        FallbackCommand {
            name: "config",
            aliases: vec![],
            before_execute: None,
            after_execute: None,
            rewrite_args: None,
        },
        FallbackCommand {
            name: "debug",
            aliases: vec![],
            before_execute: None,
            after_execute: None,
            rewrite_args: None,
        },
        FallbackCommand {
            name: "describe",
            aliases: vec!["desc", "de"],
            before_execute: None,
            after_execute: None,
            rewrite_args: None,
        },
        FallbackCommand {
            name: "diff",
            aliases: vec![],
            before_execute: None,
            after_execute: None,
            rewrite_args: None,
        },
        FallbackCommand {
            name: "diffedit",
            aliases: vec![],
            before_execute: None,
            after_execute: None,
            rewrite_args: None,
        },
        FallbackCommand {
            name: "duplicate",
            aliases: vec![],
            before_execute: None,
            after_execute: None,
            rewrite_args: None,
        },
        FallbackCommand {
            name: "edit",
            aliases: vec![],
            before_execute: None,
            after_execute: None,
            rewrite_args: None,
        },
        FallbackCommand {
            name: "evolog",
            aliases: vec![],
            before_execute: None,
            after_execute: None,
            rewrite_args: None,
        },
        FallbackCommand {
            name: "file",
            aliases: vec![],
            before_execute: None,
            after_execute: None,
            rewrite_args: None,
        },
        FallbackCommand {
            name: "fix",
            aliases: vec![],
            before_execute: None,
            after_execute: None,
            rewrite_args: None,
        },
        FallbackCommand {
            name: "git",
            aliases: vec![],
            before_execute: None,
            after_execute: None,
            rewrite_args: None,
        },
        FallbackCommand {
            name: "interdiff",
            aliases: vec![],
            before_execute: None,
            after_execute: None,
            rewrite_args: None,
        },
        FallbackCommand {
            name: "log",
            aliases: vec![],
            before_execute: None,
            after_execute: None,
            rewrite_args: None,
        },
        FallbackCommand {
            name: "new",
            aliases: vec![],
            before_execute: None,
            after_execute: None,
            rewrite_args: None,
        },
        FallbackCommand {
            name: "next",
            aliases: vec![],
            before_execute: None,
            after_execute: None,
            rewrite_args: None,
        },
        FallbackCommand {
            name: "operation",
            aliases: vec![],
            before_execute: None,
            after_execute: None,
            rewrite_args: None,
        },
        FallbackCommand {
            name: "parallelize",
            aliases: vec![],
            before_execute: None,
            after_execute: None,
            rewrite_args: None,
        },
        FallbackCommand {
            name: "prev",
            aliases: vec![],
            before_execute: None,
            after_execute: None,
            rewrite_args: None,
        },
        FallbackCommand {
            name: "rebase",
            aliases: vec![],
            before_execute: None,
            after_execute: None,
            rewrite_args: None,
        },
        FallbackCommand {
            name: "resolve",
            aliases: vec![],
            before_execute: None,
            after_execute: None,
            rewrite_args: None,
        },
        FallbackCommand {
            name: "restore",
            aliases: vec![],
            before_execute: None,
            after_execute: None,
            rewrite_args: None,
        },
        FallbackCommand {
            name: "revert",
            aliases: vec![],
            before_execute: None,
            after_execute: None,
            rewrite_args: None,
        },
        FallbackCommand {
            name: "root",
            aliases: vec![],
            before_execute: None,
            after_execute: None,
            rewrite_args: None,
        },
        FallbackCommand {
            name: "run",
            aliases: vec![],
            before_execute: None,
            after_execute: None,
            rewrite_args: None,
        },
        FallbackCommand {
            name: "show",
            aliases: vec![],
            before_execute: None,
            after_execute: None,
            rewrite_args: None,
        },
        FallbackCommand {
            name: "sign",
            aliases: vec![],
            before_execute: None,
            after_execute: None,
            rewrite_args: None,
        },
        FallbackCommand {
            name: "simplify-parents",
            aliases: vec![],
            before_execute: None,
            after_execute: None,
            rewrite_args: None,
        },
        FallbackCommand {
            name: "sparse",
            aliases: vec![],
            before_execute: None,
            after_execute: None,
            rewrite_args: None,
        },
        FallbackCommand {
            name: "split",
            aliases: vec!["sp"],
            before_execute: None,
            after_execute: None,
            rewrite_args: None,
        },
        FallbackCommand {
            name: "squash",
            aliases: vec!["sq"],
            before_execute: None,
            after_execute: None,
            rewrite_args: None,
        },
        FallbackCommand {
            name: "status",
            aliases: vec!["st"],
            before_execute: None,
            after_execute: None,
            rewrite_args: None,
        },
        FallbackCommand {
            name: "tag",
            aliases: vec!["t"],
            before_execute: None,
            after_execute: None,
            rewrite_args: None,
        },
        FallbackCommand {
            name: "undo",
            aliases: vec![],
            before_execute: None,
            after_execute: None,
            rewrite_args: None,
        },
        FallbackCommand {
            name: "unsign",
            aliases: vec![],
            before_execute: None,
            after_execute: None,
            rewrite_args: None,
        },
        FallbackCommand {
            name: "util",
            aliases: vec![],
            before_execute: None,
            after_execute: None,
            rewrite_args: None,
        },
        FallbackCommand {
            name: "version",
            aliases: vec![],
            before_execute: None,
            after_execute: None,
            rewrite_args: None,
        },
        FallbackCommand {
            name: "workspace",
            aliases: vec![],
            before_execute: None,
            after_execute: None,
            rewrite_args: None,
        },
    ]
}

/// Check if the args contain -f or --force flag
fn has_force_flag(args: &[String]) -> bool {
    args.iter().any(|arg| arg == "-f" || arg == "--force")
}

pub fn handle_fallback_command(args: &[String]) -> Result<()> {
    let fallback_commands = get_fallback_commands();
    let subcommand = &args[0];

    // Find matching command and get canonical name
    for fallback_cmd in &fallback_commands {
        if let Some(canonical_name) = fallback_cmd.matches(subcommand) {
            // Execute before_execute hook
            if let Some(before_hook) = fallback_cmd.before_execute {
                before_hook()?;
            }

            // Replace the subcommand with canonical name and keep other args
            let mut final_args = vec![canonical_name.to_string()];
            final_args.extend_from_slice(&args[1..]);

            // Rewrite args if needed
            if let Some(rewrite_fn) = fallback_cmd.rewrite_args {
                final_args = rewrite_fn(final_args)?;
            }

            // Check if -f or --force flag is present
            let has_force = has_force_flag(&final_args);

            // Build the jj command
            let mut jj_args = Vec::new();

            // Add --ignore-immutable if force flag is present
            if has_force {
                jj_args.push("--ignore-immutable".to_string());
            }

            // Add the command and its arguments
            jj_args.extend(final_args);

            // Execute the jj command
            cmd("jj", &jj_args).run()?;

            // Execute after_execute hook
            if let Some(after_hook) = fallback_cmd.after_execute {
                after_hook()?;
            }

            return Ok(());
        }
    }

    Err(anyhow!("Unknown command: {}", subcommand))
}
