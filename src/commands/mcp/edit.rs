//! Safe file edits for `edit_board` and `edit_body`: scope, dirty check, dry run, diff.

use std::path::{Path, PathBuf};

use serde_json::{json, Value};

use super::git;
use super::guard::{check_patch_scope, Restore};
use super::session::Session;
use super::tools::outcome::Outcome;

/// One requested change to a file.
pub enum Change {
    Patch(String),
    Content(String),
}

/// Runs the check after the change and returns the fuse verdict with its diagnostics.
pub type Verdict<'a> = &'a dyn Fn() -> (Value, Vec<Value>);

/// Reads exactly one of `patch` or `content` from the tool arguments.
pub fn change_from(args: &Value) -> Result<Change, String> {
    let patch = args.get("patch").and_then(Value::as_str);
    let content = args.get("content").and_then(Value::as_str);
    match (patch, content) {
        (Some(p), None) => Ok(Change::Patch(p.to_string())),
        (None, Some(c)) => Ok(Change::Content(c.to_string())),
        _ => Err("give exactly one of patch or content".into()),
    }
}

fn resolve(session: &Session, path: &Path) -> Result<(PathBuf, String), String> {
    let file = path
        .canonicalize()
        .map_err(|e| format!("{}: {e}", path.display()))?;
    let rel = file
        .strip_prefix(&session.root)
        .map_err(|_| format!("{} is outside the project root", file.display()))?;
    Ok((file.clone(), rel.to_string_lossy().into_owned()))
}

fn ensure_clean(session: &Session, file: &Path, rel: &str, original: &[u8]) -> Result<(), String> {
    if git::is_dirty(&session.root, rel)? && !session.owns(file, original) {
        return Err(format!(
            "{rel} has changes this session did not make; commit or stash them first"
        ));
    }
    Ok(())
}

fn apply(session: &Session, file: &Path, rel: &str, change: &Change) -> Result<(), String> {
    match change {
        Change::Patch(patch) => {
            check_patch_scope(patch, rel)?;
            git::apply_patch(&session.root, patch, rel)
        }
        Change::Content(text) => std::fs::write(file, text).map_err(|e| e.to_string()),
    }
}

fn run(
    session: &mut Session,
    path: &Path,
    change: &Change,
    dry: bool,
    check: Verdict,
) -> Result<Value, String> {
    let (file, rel) = resolve(session, path)?;
    let original = std::fs::read(&file).map_err(|e| e.to_string())?;
    ensure_clean(session, &file, &rel, &original)?;
    let mut guard = Restore::new(file.clone(), original, dry);
    let applied = apply(session, &file, &rel, change);
    guard.record();
    applied?;
    let (fuse, diagnostics) = check();
    let diff = git::diff(&session.root, &rel)?;
    drop(guard);
    if !dry {
        session.remember(&file);
    }
    Ok(
        json!({"applied": !dry, "dry_run": dry, "diff": diff, "fuse": fuse, "diagnostics": diagnostics}),
    )
}

/// Applies `change` to the file at `path` following the write-safety rules.
pub fn edit(
    session: &mut Session,
    path: &Path,
    change: &Change,
    dry: bool,
    check: Verdict,
) -> Outcome {
    match run(session, path, change, dry, check) {
        Ok(value) => Outcome::ok(value),
        Err(message) => Outcome::fail_with(json!({"applied": false, "error": message})),
    }
}
