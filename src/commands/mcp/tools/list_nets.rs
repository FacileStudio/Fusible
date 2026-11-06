//! Tool `list_nets`: nets of a board with their ports.

use serde_json::{json, Value};

use super::args::{object, required_str, string};
use super::outcome::Outcome;
use crate::commands::mcp::report;
use crate::commands::mcp::session::Session;
use crate::parser::{Circuit, Net};

pub const DESCRIPTION: &str = "List the nets of a board with the ports each one joins. \
The declared voltage and current stay empty until the manifest declares nets.";

/// Input schema: the board name.
pub fn schema() -> Value {
    object(&[("board", string("Board name"))], &["board"])
}

fn describe(circuit: &Circuit, net: &Net) -> Value {
    let ports: Vec<&str> = circuit
        .ports()
        .filter(|p| p.net_id.is_some() && p.net_id == net.net_id)
        .filter_map(|p| p.port_id.as_deref())
        .collect();
    let name = net.name.clone().or_else(|| net.net_id.clone());
    json!({"name": name, "ports": ports, "declared": {}})
}

/// Lists nets of the named board.
pub fn run(_session: &mut Session, args: &Value) -> Outcome {
    let board = match required_str(args, "board") {
        Ok(board) => board,
        Err(outcome) => return outcome,
    };
    let loaded = report::select(Some(board)).and_then(|targets| report::load(&targets[0]));
    match loaded {
        Ok(circuit) => {
            let rows: Vec<Value> = circuit.nets().map(|n| describe(&circuit, n)).collect();
            Outcome::ok(json!({"nets": rows}))
        }
        Err(message) => Outcome::fail(message),
    }
}
