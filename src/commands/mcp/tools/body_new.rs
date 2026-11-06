//! Tool `body_new`: scaffold a code-first body.

use serde_json::{json, Value};

use super::args::{object, optional_str, required_str, string};
use super::outcome::Outcome;
use crate::commands::body;
use crate::commands::mcp::session::Session;

pub const DESCRIPTION: &str = "Create a code-first body: mech/<name>.ts, editor types and its \
fusible.yml entry. Defaults: material petg, process fdm.";

/// Input schema: name, material, process.
pub fn schema() -> Value {
    object(
        &[
            ("name", string("Body name")),
            ("material", string("Material, such as petg")),
            (
                "process",
                string("Process: fdm, sla, mjf, injection or cnc"),
            ),
        ],
        &["name"],
    )
}

/// Scaffolds the body in the session's project.
pub fn run(session: &mut Session, args: &Value) -> Outcome {
    let name = match required_str(args, "name") {
        Ok(name) => name.to_string(),
        Err(outcome) => return outcome,
    };
    let material = optional_str(args, "material").unwrap_or("petg").to_string();
    let process = optional_str(args, "process").unwrap_or("fdm").to_string();
    let _ = std::env::set_current_dir(&session.root);
    match body::new(name.clone(), None, material, process) {
        Ok(()) => Outcome::ok(json!({"created": [format!("mech/{name}.ts"), "mech/fusible.d.ts"]})),
        Err(err) => Outcome::fail(err.to_string()),
    }
}
