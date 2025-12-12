use duct::cmd;

pub fn log(revset: &str, template: &str, ignore_working_copy: bool) -> anyhow::Result<String> {
    let mut args = vec!["log", "-r", revset, "-T", template, "--no-graph"];
    if ignore_working_copy {
        args.push("--ignore-working-copy");
    }
    let result = cmd!("jj", "log", "-r", revset, "-T", template, "--no-graph").read()?;
    Ok(result)
}

pub fn branch_exists(branch: &str) -> anyhow::Result<bool> {
    let revset = format!("bookmarks(\"{branch}\")");
    let output = log(&revset, "change_id", true)?;
    Ok(!output.trim().is_empty())
}

pub fn check_immutable(revset: &str) -> anyhow::Result<bool> {
    let revset_in_check = format!("{} & immutable()", revset);
    let output = cmd!(
        "jj",
        "log",
        "-r",
        revset_in_check,
        "-T",
        "change_id",
        "--no-graph"
    )
    .read()?;
    Ok(!output.trim().is_empty())
}
