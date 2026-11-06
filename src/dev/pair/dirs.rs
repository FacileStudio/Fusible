//! Where the paired device list lives.

use std::path::PathBuf;

use crate::config::FusibleConfig;

fn env_path(key: &str) -> Option<PathBuf> {
    std::env::var_os(key)
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
}

fn base_dir() -> Option<PathBuf> {
    if cfg!(windows) {
        env_path("LOCALAPPDATA")
    } else if cfg!(target_os = "macos") {
        env_path("HOME").map(|home| home.join("Library/Application Support"))
    } else {
        env_path("XDG_STATE_HOME").or_else(|| env_path("HOME").map(|h| h.join(".local/state")))
    }
}

/// The per-user `fusible` state directory, created when missing.
pub(super) fn state_dir() -> Option<PathBuf> {
    let dir = base_dir()?.join("fusible");
    std::fs::create_dir_all(&dir).ok()?;
    Some(dir)
}

/// The manifest name, or the name of the folder that holds the manifest.
pub(super) fn project_name() -> String {
    let Some((path, cfg)) = FusibleConfig::nearest() else {
        return "project".into();
    };
    let folder = path.parent().and_then(|dir| dir.file_name());
    let folder = folder.map(|name| name.to_string_lossy().into_owned());
    cfg.name.or(folder).unwrap_or_else(|| "project".into())
}
