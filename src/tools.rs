use duct::cmd;

pub(crate) fn get_main_branch() -> Option<String> {
    let output = cmd!(
        "jj",
        "log",
        "--no-graph",
        "-r",
        "bookmarks()",
        "-T",
        "bookmarks ++ \"\n\""
    )
    .read()
    .ok()?;
    output
        .lines()
        .find(|line| {
            line.starts_with("main") || line.starts_with("master") || line.starts_with("trunk")
        })
        .map(|line| line.to_string())
}
