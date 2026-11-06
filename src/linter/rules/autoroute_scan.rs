//! Reads tscircuit circuit elements for the `autoroute` rule.

use std::collections::{HashMap, HashSet};

use serde_json::Value;

use crate::linter::LintContext;

pub(super) const HELP: &str =
    "fix the placement or routing error; fab files from an unrouted board are unusable";
const THERMALPAD_HINT: &str =
    "a thermalpad only connects when the chip has a pinLabel named thermalpad";
pub(super) const UNROUTED: &str = "board has nets but no routed traces";

pub(super) fn type_of(el: &Value) -> Option<&str> {
    el.get("type").and_then(Value::as_str)
}

pub(super) fn error_message(el: &Value) -> String {
    let message = el
        .get("message")
        .and_then(Value::as_str)
        .unwrap_or("no message");
    format!("{}: {message}", type_of(el).unwrap_or("error"))
}

pub(super) fn error_help(el: &Value) -> String {
    let thermalpad = error_message(el).contains("thermalpad");
    match thermalpad {
        true => format!("{HELP}; {THERMALPAD_HINT}"),
        false => HELP.to_string(),
    }
}

fn has_type(raw: &[Value], pred: &dyn Fn(&str) -> bool) -> bool {
    raw.iter().filter_map(type_of).any(pred)
}

fn ids<'a>(raw: &'a [Value], kind: &'a str, key: &'a str) -> impl Iterator<Item = &'a str> {
    raw.iter()
        .filter(move |el| type_of(el) == Some(kind))
        .filter_map(move |el| el.get(key).and_then(Value::as_str))
}

fn pads_only_connect(ctx: &LintContext<'_>) -> bool {
    let raw = &ctx.circuit.raw_elements;
    if has_type(raw, &|t| t == "source_trace") {
        return false;
    }
    if has_type(raw, &|t| t == "pcb_copper_pour") {
        return true;
    }
    let mut per_net = HashMap::new();
    for port in ids(raw, "pcb_port", "source_port_id") {
        if let Some(net) = ctx.netlist.port_to_net.get(port) {
            *per_net.entry(net).or_insert(0usize) += 1;
        }
    }
    ids(raw, "pcb_port", "pcb_port_id").next().is_some() && per_net.values().all(|&n| n < 2)
}

/// True when the board has connections to make and no routed trace, and
/// nothing explains the absence: pours and adjacent pads need no trace.
pub(super) fn is_unrouted(ctx: &LintContext<'_>) -> bool {
    let raw = &ctx.circuit.raw_elements;
    has_type(raw, &|t| t.starts_with("pcb_"))
        && has_type(raw, &|t| t == "source_net" || t == "source_trace")
        && !has_type(raw, &|t| t == "pcb_trace")
        && !pads_only_connect(ctx)
}

/// The `source_trace` ids that have no `pcb_trace`, on a board that routed some traces.
pub(super) fn unrouted_traces(raw: &[Value]) -> Vec<&str> {
    let routed: HashSet<&str> = ids(raw, "pcb_trace", "source_trace_id").collect();
    if !has_type(raw, &|t| t == "pcb_trace") {
        return Vec::new();
    }
    ids(raw, "source_trace", "source_trace_id")
        .filter(|id| !routed.contains(id))
        .collect()
}
