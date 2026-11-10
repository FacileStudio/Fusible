//! Reads tscircuit circuit elements for the `autoroute` rule.

use std::collections::{HashMap, HashSet};

use crate::linter::LintContext;
use crate::parser::model::UpstreamIssue;

pub(super) const HELP: &str =
    "fix the placement or routing error; fab files from an unrouted board are unusable";
const THERMALPAD_HINT: &str =
    "a thermalpad only connects when the chip has a pinLabel named thermalpad";
pub(super) const UNROUTED: &str = "board has nets but no routed traces";

pub(super) fn error_message(issue: &UpstreamIssue) -> String {
    let message = issue.message.as_deref().unwrap_or("no message");
    format!("{}: {message}", issue.kind)
}

pub(super) fn error_help(issue: &UpstreamIssue) -> String {
    let thermalpad = error_message(issue).contains("thermalpad");
    match thermalpad {
        true => format!("{HELP}; {THERMALPAD_HINT}"),
        false => HELP.to_string(),
    }
}

fn pads_only_connect(ctx: &LintContext<'_>) -> bool {
    let model = ctx.circuit.model();
    if model.count("source_trace") > 0 {
        return false;
    }
    if model.count("pcb_copper_pour") > 0 {
        return true;
    }
    let mut per_net = HashMap::new();
    let ports = ctx.circuit.pcb_ports();
    for port in ports.filter_map(|p| p.source_port_id.as_deref()) {
        if let Some(net) = ctx.netlist.port_to_net.get(port) {
            *per_net.entry(net).or_insert(0usize) += 1;
        }
    }
    let any_port = ctx.circuit.pcb_ports().any(|p| p.pcb_port_id.is_some());
    any_port && per_net.values().all(|&n| n < 2)
}

/// True when the board has connections to make and no routed trace, and
/// nothing explains the absence: pours and adjacent pads need no trace.
pub(super) fn is_unrouted(ctx: &LintContext<'_>) -> bool {
    let model = ctx.circuit.model();
    model.has_type(|t| t.starts_with("pcb_"))
        && model.has_type(|t| t == "source_net" || t == "source_trace")
        && model.count("pcb_trace") == 0
        && !pads_only_connect(ctx)
}

/// The `source_trace` ids that have no `pcb_trace`, on a board that routed some traces.
pub(super) fn unrouted_traces<'a>(ctx: &LintContext<'a>) -> Vec<&'a str> {
    let model = ctx.circuit.model();
    if model.count("pcb_trace") == 0 {
        return Vec::new();
    }
    let traces = model.traces.iter();
    let routed: HashSet<&str> = traces
        .filter_map(|t| t.source_trace_id.as_deref())
        .collect();
    let sources = ctx.circuit.traces().filter_map(|t| t.trace_id.as_deref());
    sources.filter(|id| !routed.contains(id)).collect()
}
