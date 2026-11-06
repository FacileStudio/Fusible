//! Tool `project_info`: boards, firmware, bodies and git state.

use std::path::Path;

use serde_json::{json, Value};

use super::args::object;
use super::outcome::Outcome;
use crate::commands::mcp::git;
use crate::commands::mcp::session::Session;
use crate::config::{BoardConfig, BodyConfig, FirmwareConfig, FusibleConfig};

pub const DESCRIPTION: &str =
    "Describe the project: boards, firmware, bodies and git state. Call this first.";

/// Input schema: no arguments.
pub fn schema() -> Value {
    object(&[], &[])
}

fn relative(root: &Path, path: Option<&Path>) -> Value {
    let shown = path.map(|p| {
        p.strip_prefix(root)
            .unwrap_or(p)
            .to_string_lossy()
            .into_owned()
    });
    json!(shown)
}

fn body_kind(body: &BodyConfig) -> &'static str {
    let code = body
        .source
        .as_deref()
        .and_then(Path::extension)
        .is_some_and(|e| e == "ts");
    match (&body.generate, code) {
        (Some(_), _) => "generated",
        (None, true) => "code",
        (None, false) => "authored",
    }
}

fn boards(cfg: &FusibleConfig, root: &Path) -> Vec<Value> {
    let row = |(name, b): (&String, &BoardConfig)| {
        let entry = b.entry.as_deref().or(b.circuit_json.as_deref());
        json!({"name": name, "entry": relative(root, entry), "mcu": b.mcu})
    };
    cfg.boards.iter().map(row).collect()
}

fn firmware(cfg: &FusibleConfig) -> Vec<Value> {
    let row = |(name, f): (&String, &FirmwareConfig)| json!({"name": name, "board": f.board, "framework": f.framework});
    cfg.firmware.iter().map(row).collect()
}

fn bodies(cfg: &FusibleConfig) -> Vec<Value> {
    let row = |(name, b): (&String, &BodyConfig)| {
        let (material, process) = (Value::Null, Value::Null);
        json!({"name": name, "kind": body_kind(b), "material": material, "process": process})
    };
    cfg.bodies.iter().map(row).collect()
}

/// Reports the project the server was started on.
pub fn run(session: &mut Session, _args: &Value) -> Outcome {
    let cfg = match session.config() {
        Ok(Some(cfg)) => cfg,
        Ok(None) => return Outcome::ok(json!({"manifest": null, "root": session.root})),
        Err(message) => return Outcome::fail(message),
    };
    let name = cfg.name.clone().or_else(|| {
        session
            .root
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
    });
    let branch = git::branch(&session.root);
    let git =
        json!({"branch": branch, "dirty": branch.is_some() && git::tree_dirty(&session.root)});
    Outcome::ok(json!({
        "name": name,
        "root": session.root,
        "boards": boards(&cfg, &session.root),
        "firmware": firmware(&cfg),
        "bodies": bodies(&cfg),
        "git": git,
    }))
}
