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
