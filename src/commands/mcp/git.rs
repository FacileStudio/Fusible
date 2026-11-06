//! The few git commands the edit tools need.

use std::path::{Path, PathBuf};
use std::process::Command;

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

fn temp_patch() -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_nanos());
    std::env::temp_dir().join(format!("fusible-mcp-{}-{nanos}.patch", std::process::id()))
}

/// Checks the patch, then applies it from `root`.
pub fn apply_patch(root: &Path, patch: &str) -> Result<(), String> {
    let path = temp_patch();
    std::fs::write(&path, patch).map_err(|e| e.to_string())?;
    let file = path.to_string_lossy().into_owned();
    let result = run(root, &["apply", "--check", &file]).and_then(|_| run(root, &["apply", &file]));
    let _ = std::fs::remove_file(&path);
    result.map(|_| ())
}
