use std::env;
use std::process::{Command, ExitCode};

fn run(args: &[&str]) -> Option<String> {
    let out = Command::new("git").args(args).output().ok()?;
    if !out.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

fn main() -> ExitCode {
    if env::args().any(|a| a == "-h" || a == "--help") {
        println!(
            "gitsum — quick snapshot of the current git repo\n\nUsage:\n  gitsum\n"
        );
        return ExitCode::SUCCESS;
    }

    if run(&["rev-parse", "--is-inside-work-tree"]).as_deref() != Some("true") {
        eprintln!("error: not a git repository");
        return ExitCode::from(1);
    }

    let root = run(&["rev-parse", "--show-toplevel"]).unwrap_or_else(|| ".".into());
    let branch = run(&["rev-parse", "--abbrev-ref", "HEAD"]).unwrap_or_else(|| "?".into());
    let short = run(&["rev-parse", "--short", "HEAD"]).unwrap_or_else(|| "?".into());
    let ahead_behind = run(&[
        "rev-list",
        "--left-right",
        "--count",
        "@{upstream}...HEAD",
    ]);
    let status = run(&["status", "--porcelain"]).unwrap_or_default();
    let commits = run(&["rev-list", "--count", "HEAD"]).unwrap_or_else(|| "?".into());
    let authors = run(&["shortlog", "-sn", "HEAD"]).unwrap_or_default();
    let last = run(&["log", "-1", "--format=%cr · %s"]).unwrap_or_else(|| "?".into());

    let mut staged = 0;
    let mut unstaged = 0;
    let mut untracked = 0;
    for line in status.lines() {
        let b = line.as_bytes();
        if b.len() < 2 {
            continue;
        }
        if b[0] == b'?' {
            untracked += 1;
        } else {
            if b[0] != b' ' && b[0] != b'?' {
                staged += 1;
            }
            if b[1] != b' ' && b[1] != b'?' {
                unstaged += 1;
            }
        }
    }

    let name = root.rsplit('/').next().unwrap_or(&root);
    println!("📦 {name}");
    println!("   branch   {branch} @ {short}");
    println!("   commits  {commits}");
    println!("   last     {last}");
    if let Some(ab) = ahead_behind {
        let parts: Vec<&str> = ab.split_whitespace().collect();
        if parts.len() == 2 {
            println!("   sync     ↑{} ↓{}", parts[1], parts[0]);
        }
    } else {
        println!("   sync     (no upstream)");
    }
    println!("   dirty    staged={staged} unstaged={unstaged} untracked={untracked}");

    let top: Vec<&str> = authors.lines().take(3).collect();
    if !top.is_empty() {
        println!("   authors");
        for line in top {
            println!("            {line}");
        }
    }

    ExitCode::SUCCESS
}
