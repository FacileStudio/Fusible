//! Every `fusible` binary on `PATH`, to catch a stale install shadowing the current one.

use std::ffi::OsStr;
use std::path::PathBuf;

use serde_json::{json, Value};

use super::json::version;

const FIX: &str = "remove the older one (mise uninstall cargo:fusible) or reorder PATH";

/// One `fusible` found on `PATH`; `active` is the one a shell would run.
pub struct Install {
    pub path: PathBuf,
    pub version: Option<String>,
    pub active: bool,
}

/// Lists every distinct `fusible` file in the directories of `path_var`, in `PATH` order.
pub fn find(path_var: &OsStr) -> Vec<Install> {
    let name = if cfg!(windows) {
        "fusible.exe"
    } else {
        "fusible"
    };
    let mut seen: Vec<PathBuf> = Vec::new();
    let mut installs = Vec::new();
    for dir in std::env::split_paths(path_var) {
        let file = dir.join(name);
        let real = file.canonicalize().unwrap_or_else(|_| file.clone());
        if file.is_file() && !seen.contains(&real) {
            seen.push(real);
            let version = version(&file).map(|v| v.trim_start_matches("fusible ").to_string());
            installs.push(Install {
                path: file,
                version,
                active: installs.is_empty(),
            });
        }
    }
    installs
}

/// Returns the warning and its fix when more than one install exists.
pub fn warning(installs: &[Install]) -> Option<(String, &'static str)> {
    let (first, rest) = installs.split_first()?;
    if rest.is_empty() {
        return None;
    }
    let show = |i: &Install| {
        format!(
            "{} ({})",
            i.path.display(),
            i.version.as_deref().unwrap_or("unknown")
        )
    };
    let shadowed: Vec<String> = rest.iter().map(show).collect();
    let message = format!(
        "{} fusible binaries on PATH: {} shadows {}",
        installs.len(),
        show(first),
        shadowed.join(", ")
    );
    Some((message, FIX))
}

/// Builds the `path_installs` JSON array.
pub fn to_json(installs: &[Install]) -> Value {
    let rows = installs.iter().map(|i| {
        json!({ "path": i.path.display().to_string(), "version": i.version, "active": i.active })
    });
    Value::Array(rows.collect())
}
