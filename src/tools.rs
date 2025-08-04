use duct::cmd;

pub fn log(revset: &str, template: &str) -> anyhow::Result<String> {
    let result = cmd!("jj", "log", "-r", revset, "-T", template, "--no-graph").read()?;
    Ok(result)
}
