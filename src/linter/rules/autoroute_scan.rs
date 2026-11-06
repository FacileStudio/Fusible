//! Reads tscircuit circuit elements for the `autoroute` rule.

use serde_json::Value;

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

pub(super) fn is_unrouted(raw: &[Value]) -> bool {
    let has = |pred: &dyn Fn(&str) -> bool| raw.iter().filter_map(type_of).any(pred);
    has(&|t| t.starts_with("pcb_"))
        && has(&|t| t == "source_net" || t == "source_trace")
        && !has(&|t| t == "pcb_trace")
}
