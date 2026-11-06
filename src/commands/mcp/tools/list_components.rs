//! Tool `list_components`: components of a board with their ports.

use serde_json::{json, Value};

use super::args::{object, required_str, string};
use super::outcome::Outcome;
use crate::commands::mcp::report;
use crate::commands::mcp::session::Session;
use crate::parser::{Circuit, Component, Port};

pub const DESCRIPTION: &str = "List the components of a board with their kind and ports.";

/// Input schema: the board name.
pub fn schema() -> Value {
    object(&[("board", string("Board name"))], &["board"])
}

fn belongs(port: &Port, id: &str) -> bool {
    let prefix = format!("{id}.");
    let by_owner = port.source_component_id.as_deref() == Some(id);
    by_owner
        || port
            .port_id
            .as_deref()
            .is_some_and(|p| p.starts_with(&prefix))
}

fn port_label(port: &Port) -> String {
    let id = port.port_id.clone().unwrap_or_default();
    match &port.name {
        Some(name) => format!("{id} ({name})"),
        None => id,
    }
}

fn describe(circuit: &Circuit, component: &Component) -> Value {
    let id = component.component_id.clone().unwrap_or_default();
    let ports: Vec<String> = circuit
        .ports()
        .filter(|p| belongs(p, &id))
        .map(port_label)
        .collect();
    let name = component.name.clone().unwrap_or_else(|| id.clone());
    json!({"name": name, "kind": component.kind, "ports": ports})
}

/// Lists components of the named board.
pub fn run(_session: &mut Session, args: &Value) -> Outcome {
    let board = match required_str(args, "board") {
        Ok(board) => board,
        Err(outcome) => return outcome,
    };
    let loaded = report::select(Some(board)).and_then(|targets| report::load(&targets[0]));
    match loaded {
        Ok(circuit) => {
            let rows: Vec<Value> = circuit
                .components()
                .map(|c| describe(&circuit, c))
                .collect();
            Outcome::ok(json!({"components": rows}))
        }
        Err(message) => Outcome::fail(message),
    }
}
