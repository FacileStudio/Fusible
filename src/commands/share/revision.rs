//! The revision label of a shared export.

use std::process::Command;

fn git(args: &[&str]) -> Option<String> {
    let output = Command::new("git").args(args).output().ok()?;
    let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
    (output.status.success()).then_some(text)
}

/// Short commit hash, with `-dirty` for uncommitted changes, or the crate version outside git.
pub fn current() -> String {
    let Some(short) = git(&["rev-parse", "--short", "HEAD"]) else {
        return env!("CARGO_PKG_VERSION").to_string();
    };
    match git(&["status", "--porcelain"]).is_some_and(|s| !s.is_empty()) {
        true => format!("{short}-dirty"),
        false => short,
    }
}
