//! Restore-on-drop for dry runs, and patch scope validation.

use std::path::PathBuf;

/// Writes the original bytes back when dropped, unless built disarmed.
pub struct Restore {
    path: PathBuf,
    bytes: Vec<u8>,
    armed: bool,
}

impl Restore {
    /// Guards `path`; the bytes return to disk on drop when `armed`.
    pub fn new(path: PathBuf, bytes: Vec<u8>, armed: bool) -> Self {
        Self { path, bytes, armed }
    }
}

impl Drop for Restore {
    fn drop(&mut self) {
        if self.armed {
            let _ = std::fs::write(&self.path, &self.bytes);
        }
    }
}

fn header_path(line: &str) -> Option<String> {
    let rest = line
        .strip_prefix("--- ")
        .or_else(|| line.strip_prefix("+++ "))?;
    let name = rest.split('\t').next().unwrap_or(rest).trim();
    Some(
        name.strip_prefix("a/")
            .or_else(|| name.strip_prefix("b/"))
            .unwrap_or(name)
            .to_string(),
    )
}

fn git_paths(line: &str) -> Vec<String> {
    let rest = line.strip_prefix("diff --git ").unwrap_or_default();
    let strip = |p: &str| {
        p.trim_start_matches("a/")
            .trim_start_matches("b/")
            .to_string()
    };
    rest.split(' ').map(strip).collect()
}

/// Refuses a patch that touches any file other than `rel`, or that renames or copies.
pub fn check_patch_scope(patch: &str, rel: &str) -> Result<(), String> {
    let mut seen = 0;
    for line in patch.lines() {
        let names = match line {
            l if l.starts_with("diff --git ") => git_paths(l),
            l if l.starts_with("rename ") || l.starts_with("copy ") => {
                return Err("the patch renames or copies files".into());
            }
            l => header_path(l).into_iter().collect(),
        };
        seen += names.len();
        if let Some(other) = names.iter().find(|n| n.as_str() != rel) {
            return Err(format!(
                "the patch touches \"{other}\"; it may only change {rel}"
            ));
        }
    }
    (seen > 0)
        .then_some(())
        .ok_or_else(|| format!("the patch is not a unified diff of {rel}"))
}
