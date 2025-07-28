use duct::cmd;

pub fn log(revset: &str, template: &str) -> Option<String> {
    cmd!("jj", "log", "-r", revset, "-T", template, "--no-graph")
        .read()
        .ok()
}
