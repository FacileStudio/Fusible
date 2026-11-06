//! `autoroute`: tscircuit placement and routing failures must blow the fuse.

use crate::linter::{Diagnostic, LintContext, Rule};

use super::autoroute_scan::{
    error_help, error_message, is_unrouted, type_of, unrouted_traces, HELP, UNROUTED,
};

/// tscircuit records a failed placement or autoroute as an `*_error` element and keeps going.
/// Without this rule the fuse stays intact on a board whose copper was never routed.
pub struct AutorouteRule;

impl Rule for AutorouteRule {
    fn name(&self) -> &'static str {
        "autoroute"
    }

    fn summary(&self) -> &'static str {
        "Blows the fuse when tscircuit reports a placement or routing error, or leaves nets with no routed trace."
    }

    fn standard(&self) -> Option<&'static str> {
        Some("tscircuit")
    }

    fn check(&self, ctx: &LintContext<'_>) -> Vec<Diagnostic> {
        let raw = &ctx.circuit.raw_elements;
        let mut found: Vec<Diagnostic> = raw
            .iter()
            .filter(|el| type_of(el).is_some_and(|t| t.ends_with("_error")))
            .map(|el| self.diagnostic(error_message(el), error_help(el)))
            .collect();
        if is_unrouted(ctx) {
            found.push(self.diagnostic(UNROUTED.to_string(), HELP.to_string()));
        }
        let missing = unrouted_traces(raw);
        if !missing.is_empty() {
            let message = format!(
                "source_trace without a routed pcb_trace: {}",
                missing.join(", ")
            );
            found.push(self.diagnostic(message, HELP.to_string()));
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
