//! `autoroute`: tscircuit placement and routing failures must blow the fuse.

use serde_json::Value;

use crate::linter::{Diagnostic, LintContext, Rule};

const HELP: &str =
    "fix the placement or routing error; fab files from an unrouted board are unusable";
const THERMALPAD_HINT: &str =
    "a thermalpad only connects when the chip has a pinLabel named thermalpad";
const UNROUTED: &str = "board has nets but no routed traces";

/// tscircuit records a failed placement or autoroute as an `*_error` element and keeps going.
/// Without this rule the fuse stays intact on a board whose copper was never routed.
pub struct AutorouteRule;

impl Rule for AutorouteRule {
    fn name(&self) -> &'static str {
        "autoroute"
    }

    fn check(&self, ctx: &LintContext<'_>) -> Vec<Diagnostic> {
        let raw = &ctx.circuit.raw_elements;
        let mut found: Vec<Diagnostic> = raw
            .iter()
            .filter(|el| type_of(el).is_some_and(|t| t.ends_with("_error")))
            .map(|el| self.diagnostic(error_message(el), error_help(el)))
            .collect();
        if is_unrouted(raw) {
            found.push(self.diagnostic(UNROUTED.to_string(), HELP.to_string()));
        }
        found
    }
}

impl AutorouteRule {
    fn diagnostic(&self, message: String, help: String) -> Diagnostic {
        Diagnostic {
            rule: self.name().to_string(),
            message,
            help: Some(help),
            ..Default::default()
        }
    }
}

fn type_of(el: &Value) -> Option<&str> {
    el.get("type").and_then(Value::as_str)
}

fn error_message(el: &Value) -> String {
    let message = el
        .get("message")
        .and_then(Value::as_str)
        .unwrap_or("no message");
    format!("{}: {message}", type_of(el).unwrap_or("error"))
}

fn error_help(el: &Value) -> String {
    let thermalpad = error_message(el).contains("thermalpad");
    match thermalpad {
        true => format!("{HELP}; {THERMALPAD_HINT}"),
        false => HELP.to_string(),
    }
}

fn is_unrouted(raw: &[Value]) -> bool {
    let has = |pred: &dyn Fn(&str) -> bool| raw.iter().filter_map(type_of).any(pred);
    has(&|t| t.starts_with("pcb_"))
        && has(&|t| t == "source_net" || t == "source_trace")
        && !has(&|t| t == "pcb_trace")
}
