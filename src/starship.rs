use duct::cmd;

/// find by tracing the closest bookmark, 18 ~ 30 ms
pub fn current_branch() -> Option<String> {
    cmd!(
        "jj",
        "log",
        "-r",
        "heads(::@- & bookmarks())",
        "-T",
        "bookmarks",
        "--no-graph"
    )
    .read()
    .map(|s| s.replace(" ", "/"))
    .ok()
}

mod test {
    use super::*;

    #[test]
    fn test_current_branch() {
        let start_time = std::time::Instant::now();
        for _ in 0..1 {
            current_branch();
        }
        println!("time: {:?}", start_time.elapsed());
        println!("{:?}", current_branch());
    }
}
