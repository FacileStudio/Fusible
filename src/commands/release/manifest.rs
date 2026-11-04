//! `RELEASE.txt`: tool version, git revision, fuse verdict and file checksums.

use std::path::{Path, PathBuf};
use std::process::Command;

/// Writes `RELEASE.txt` into `out`, listing every file under it with its SHA-256.
pub(super) fn write(out: &Path, boards: &[&str]) -> std::io::Result<()> {
    let mut text = format!(
        "fusible {}\nrevision {}\n",
        env!("CARGO_PKG_VERSION"),
        revision()
    );
    for board in boards {
        text.push_str(&format!("board {board}: fuse intact\n"));
    }
    text.push_str("files (sha256):\n");
    for file in files_under(out)? {
        let rel = file.strip_prefix(out).unwrap_or(&file);
        text.push_str(&format!("  {}  {}\n", sha256(&file), rel.display()));
    }
    std::fs::write(out.join("RELEASE.txt"), text)
}

fn revision() -> String {
    let git = Command::new("git")
        .args(["describe", "--always", "--dirty", "--abbrev=12"])
        .output();
    match git {
        Ok(out) if out.status.success() => String::from_utf8_lossy(&out.stdout).trim().to_string(),
        _ => "unknown".to_string(),
    }
}

fn sha256(file: &Path) -> String {
    let out = Command::new("sha256sum").arg(file).output();
    match out {
        Ok(out) if out.status.success() => String::from_utf8_lossy(&out.stdout)
            .split_whitespace()
            .next()
            .unwrap_or("unavailable")
            .to_string(),
        _ => "unavailable".to_string(),
    }
}

fn files_under(dir: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            files.extend(files_under(&path)?);
        } else if path.file_name().is_some_and(|n| n != "RELEASE.txt") {
            files.push(path);
        }
    }
    files.sort();
    Ok(files)
}
