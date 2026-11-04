//! Machine-readable toolchain report for `fusible doctor --json`.

use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::{json, Value};

use crate::tscircuit::detect::find_binary_in_path;
use crate::tscircuit::{bun_path, Runner, RunnerKind};

/// Builds `{"bun", "node", "tsci", "path_installs"}`: tools are `{path, version}` or null when missing;
/// `path_installs` lists every `fusible` on `PATH` as `{path, version, active}`.
pub fn report() -> Value {
    let path_var = std::env::var_os("PATH");
    let on_path = |name: &str| {
        path_var
            .as_deref()
            .and_then(|p| find_binary_in_path(name, p))
    };
    let installs = path_var
        .as_deref()
        .map(super::installs::find)
        .unwrap_or_default();
    json!({
        "bun": tool(bun_path()),
        "node": tool(on_path("node")),
        "tsci": tool(tsci_path()),
        "path_installs": super::installs::to_json(&installs),
    })
}

fn tsci_path() -> Option<PathBuf> {
    match Runner::detect().ok()?.kind() {
        RunnerKind::Local(path) | RunnerKind::Global(path) => Some(path.clone()),
        RunnerKind::Bun(_) => None,
    }
}

fn tool(path: Option<PathBuf>) -> Value {
    match path {
        Some(path) => json!({ "path": path.display().to_string(), "version": version(&path) }),
        None => Value::Null,
    }
}

pub(super) fn version(path: &Path) -> Option<String> {
    let out = Command::new(path).arg("--version").output().ok()?;
    let text = String::from_utf8_lossy(&out.stdout);
    let line = text.lines().next()?.trim();
    (out.status.success() && !line.is_empty()).then(|| line.to_string())
}
