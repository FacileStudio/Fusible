//! The few git commands the edit tools need.

use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

fn run(root: &Path, args: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .map_err(|e| format!("cannot run git: {e}"))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("git {}: {}", args.join(" "), stderr.trim()));
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

/// Whether git reports changes for `rel` (a path relative to `root`).
pub fn is_dirty(root: &Path, rel: &str) -> Result<bool, String> {
    run(root, &["status", "--porcelain", "--", rel]).map(|out| !out.trim().is_empty())
}

/// The uncommitted diff of `rel`.
pub fn diff(root: &Path, rel: &str) -> Result<String, String> {
    run(root, &["diff", "--no-color", "--", rel])
}

/// The current branch, or `None` outside a repository.
pub fn branch(root: &Path) -> Option<String> {
    let out = run(root, &["rev-parse", "--abbrev-ref", "HEAD"]).ok()?;
    Some(out.trim().to_string())
}

/// Whether the working tree has any change.
pub fn tree_dirty(root: &Path) -> bool {
    run(root, &["status", "--porcelain"]).is_ok_and(|out| !out.trim().is_empty())
}

fn run_stdin(root: &Path, args: &[&str], input: &[u8]) -> Result<Vec<u8>, String> {
    let mut child = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("cannot run git: {e}"))?;
    let mut stdin = child.stdin.take().ok_or("git stdin is closed")?;
    let data = input.to_vec();
    let writer = std::thread::spawn(move || stdin.write_all(&data));
    let output = child.wait_with_output().map_err(|e| e.to_string())?;
    let _ = writer.join();
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("git {}: {}", args.join(" "), stderr.trim()));
    }
    Ok(output.stdout)
}

/// Every path `git apply` would write for `patch`, as git itself parses it.
pub fn patch_targets(root: &Path, patch: &str) -> Result<Vec<String>, String> {
    let out = run_stdin(root, &["apply", "--numstat", "-z"], patch.as_bytes())?;
    let text = String::from_utf8_lossy(&out);
    let paths = text
        .split('\0')
        .map(|record| record.splitn(3, '\t').nth(2).unwrap_or(record));
    Ok(paths
        .filter(|p| !p.is_empty())
        .map(str::to_string)
        .collect())
}

/// Checks that git targets only `rel`, then applies the patch from `root`.
pub fn apply_patch(root: &Path, patch: &str, rel: &str) -> Result<(), String> {
    let targets = patch_targets(root, patch)?;
    if let Some(other) = targets.iter().find(|t| t.as_str() != rel) {
        return Err(format!(
            "the patch writes \"{other}\"; it may only change {rel}"
        ));
    }
    run_stdin(root, &["apply", "--check"], patch.as_bytes())?;
    run_stdin(root, &["apply"], patch.as_bytes()).map(|_| ())
}

#[cfg(test)]
#[path = "git_tests.rs"]
mod tests;
