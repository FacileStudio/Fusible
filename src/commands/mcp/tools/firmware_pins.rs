//! Tool `firmware_pins`: generated pin bindings, only behind an intact fuse.

use serde_json::{json, Value};

use super::args::{object, required_str, string};
use super::outcome::Outcome;
use crate::codegen::{extract_mcu_pins, generate_pins_rs, McuInfo};
use crate::commands::mcp::report;
use crate::commands::mcp::session::Session;
use crate::netlist::Netlist;

pub const DESCRIPTION: &str = "Generate the Rust pin bindings (pins.rs) of a firmware. \
Refuses while the fuse of its board is blown.";

/// Input schema: the firmware name.
pub fn schema() -> Value {
    object(&[("firmware", string("Firmware name"))], &["firmware"])
}

fn table(info: &McuInfo) -> Vec<Value> {
    let row = |p: &crate::codegen::PinMapping| {
        json!({"pin": p.pin_number, "label": p.pin_label, "signal": p.signal_name, "net": p.net_name,
               "remote_component": p.remote_component, "remote_port": p.remote_port})
    };
    info.pins.iter().map(row).collect()
}

fn board_of(session: &Session, firmware: &str) -> Result<Option<String>, String> {
    let cfg = session.config()?.ok_or("the project has no manifest")?;
    let fw = cfg
        .firmware
        .get(firmware)
        .ok_or_else(|| format!("unknown firmware \"{firmware}\""))?;
    Ok(fw.board.clone())
}

fn generate(board: Option<String>) -> Outcome {
    let target = match report::select(board.as_deref()) {
        Ok(mut targets) if targets.len() == 1 => targets.remove(0),
        Ok(_) => return Outcome::fail("the firmware names no board and the project has several"),
        Err(message) => return Outcome::fail(message),
    };
    let lint = report::lint_board(&target);
    if lint.blown {
        return Outcome::fail_with(
            json!({"fuse": "blown", "violations": report::diagnostics_json(&lint)}),
        );
    }
    let circuit = match report::load(&target) {
        Ok(circuit) => circuit,
        Err(message) => return Outcome::fail(message),
    };
    let netlist = Netlist::from_circuit(&circuit);
    match extract_mcu_pins(&circuit, &netlist, target.mcu.as_deref()) {
        Some(info) => Outcome::ok(
            json!({"pins_rs": generate_pins_rs(&info, &target.title), "pins": table(&info)}),
        ),
        None => Outcome::fail("no microcontroller found on the board"),
    }
}

/// Generates pin bindings for the named firmware.
pub fn run(session: &mut Session, args: &Value) -> Outcome {
    let firmware = match required_str(args, "firmware") {
        Ok(name) => name,
        Err(outcome) => return outcome,
    };
    match board_of(session, firmware) {
        Ok(board) => generate(board),
        Err(message) => Outcome::fail(message),
    }
}
