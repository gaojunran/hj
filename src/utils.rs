use console::style;

pub(crate) fn hint(message: &str) {
    println!("{}{}", style("Hint: ").blue().bold(), style(message).blue());
}

pub(crate) fn error(message: &str) {
    eprintln!("{}{}", style("Error: ").red().bold(), style(message).red());
}

pub(crate) fn warning(message: &str) {
    eprintln!(
        "{}{}",
        style("Warning: ").yellow().bold(),
        style(message).yellow()
    );
}

pub(crate) fn step(message: &str) {
    println!("🔥 {}", style(message).blue().bold());
}

/// Build a vector of jj command arguments, filtering out empty strings.
/// This is useful for conditionally adding arguments.
///
/// # Example
/// ```ignore
/// let args = build_jj_args(&[
///     "squash",
///     "--interactive",
///     if force { "--ignore-immutable" } else { "" },
/// ]);
/// ```
pub(crate) fn build_jj_args<'a>(args: &[&'a str]) -> Vec<&'a str> {
    args.iter().filter(|s| !s.is_empty()).copied().collect()
}
