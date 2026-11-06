//! Staging and copying the export: unique temp folder, name checks, symlink-free copy.

use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use super::ShareError;

static COUNTER: AtomicU64 = AtomicU64::new(0);

/// A fresh empty folder under the temp dir, private to the current user.
pub fn unique_dir() -> std::io::Result<PathBuf> {
    loop {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_nanos());
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        let name = format!("fusible-share-{}-{nanos}-{n}", std::process::id());
        let path = std::env::temp_dir().join(name);
        match private_dir(&path) {
            Ok(()) => return Ok(path),
            Err(e) if e.kind() == ErrorKind::AlreadyExists => continue,
            Err(e) => return Err(e),
        }
    }
}

#[cfg(unix)]
fn private_dir(path: &Path) -> std::io::Result<()> {
    let mut builder = std::fs::DirBuilder::new();
    std::os::unix::fs::DirBuilderExt::mode(&mut builder, 0o700);
    builder.create(path)
}

#[cfg(not(unix))]
fn private_dir(path: &Path) -> std::io::Result<()> {
    std::fs::create_dir(path)
}

/// Refuses a project name that could leave its folder.
pub fn valid_name(name: &str) -> Result<(), ShareError> {
    let bad = name.is_empty()
        || name.starts_with('.')
        || name.contains("..")
        || name.contains(['/', '\\', '\0']);
    match bad {
        true => Err(ShareError::BadName(name.replace('\0', ""))),
        false => Ok(()),
    }
}

/// Copies the tree at `from` into `to`, refusing symlinks instead of following them.
pub fn copy_tree(from: &Path, to: &Path) -> std::io::Result<usize> {
    std::fs::create_dir_all(to)?;
    let mut count = 0;
    for entry in std::fs::read_dir(from)? {
        let entry = entry?;
        let kind = entry.file_type()?;
        let target = to.join(entry.file_name());
        count += match (kind.is_symlink(), kind.is_dir()) {
            (true, _) => {
                let msg = format!("refusing symlink {}", entry.path().display());
                return Err(std::io::Error::new(ErrorKind::InvalidInput, msg));
            }
            (_, true) => copy_tree(&entry.path(), &target)?,
            _ => std::fs::copy(entry.path(), &target).map(|_| 1)?,
        };
    }
    Ok(count)
}

/// The revision folder; `force` empties that folder only, never its parents.
pub fn destination(
    sync_dir: &Path,
    project: &str,
    rev: &str,
    force: bool,
) -> Result<PathBuf, ShareError> {
    let dest = sync_dir.join("Fusible").join(project).join(rev);
    let occupied = std::fs::read_dir(&dest).is_ok_and(|mut entries| entries.next().is_some());
    match (occupied, force) {
        (true, false) => Err(ShareError::Exists(dest)),
        (true, true) => std::fs::remove_dir_all(&dest)
            .map(|_| dest)
            .map_err(Into::into),
        _ => Ok(dest),
    }
}

#[cfg(test)]
#[path = "stage_tests.rs"]
mod tests;
