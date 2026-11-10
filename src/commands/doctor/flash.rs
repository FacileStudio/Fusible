//! The flash tools `fusible flash` shells out to: `probe-rs` and `espflash`.

use std::path::PathBuf;

use serde_json::{json, Value};

use crate::tscircuit::detect::find_binary_in_path;

/// Tool names, as `doctor --json` keys them, and their binaries.
const TOOLS: [(&str, &str); 2] = [("probe_rs", "probe-rs"), ("espflash", "espflash")];

fn find(binary: &str) -> Option<PathBuf> {
    find_binary_in_path(binary, &std::env::var_os("PATH")?)
}

/// Prints where each flash tool is, or that it is missing.
pub fn print() {
    println!("\nFlash tools:");
    for (_, binary) in TOOLS {
        super::print_status(binary, find(binary).as_deref());
    }
}

/// `{"probe_rs": {path, version} | null, "espflash": ...}` entries for `doctor --json`.
pub fn entries() -> Vec<(&'static str, Value)> {
    TOOLS
        .iter()
        .map(|(key, binary)| {
            let entry = find(binary).map_or(Value::Null, |path| {
                json!({ "path": path.display().to_string(), "version": super::json::version(&path) })
            });
            (*key, entry)
        })
        .collect()
}
