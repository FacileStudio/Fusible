//! `qfn_footprint_span`: a bare qfn footprint string describes copper span, not body size.

use serde_json::Value;

use crate::linter::{Diagnostic, LintContext, Rule, Severity};

const MIN_SAFE_SPAN_MM: f64 = 3.5;
const MESSAGE: &str = "qfn w/h is the copper span, not the body; corner pads may short";
const HELP: &str = "set explicit pad size, e.g. qfn16_w3.8_h3.8_p0.5_pw0.25_pl0.8";

/// Warns on `qfn<N>_w<W>_h<H>_p<P>` footprints with no `_pw`/`_pl` pad size and a small span.
pub struct QfnFootprintSpanRule;

impl Rule for QfnFootprintSpanRule {
    fn name(&self) -> &'static str {
        "qfn_footprint_span"
    }

    fn summary(&self) -> &'static str {
        "Warns when a qfn footprint gives only a copper span, which can short the corner pads."
    }

    fn standard(&self) -> Option<&'static str> {
        Some("tscircuit footprinter")
    }

    fn severity(&self) -> Severity {
        Severity::Warning
    }

    fn check(&self, ctx: &LintContext<'_>) -> Vec<Diagnostic> {
        let mut seen: Vec<&str> = Vec::new();
        for footprint in ctx.circuit.raw_elements.iter().filter_map(footprint_of) {
            if is_risky(footprint) && !seen.contains(&footprint) {
                seen.push(footprint);
            }
        }
        seen.into_iter()
            .map(|footprint| Diagnostic {
                rule: self.name().to_string(),
                message: format!("{MESSAGE} ({footprint})"),
                help: Some(HELP.to_string()),
                ..Default::default()
            })
            .collect()
    }
}

fn footprint_of(el: &Value) -> Option<&str> {
    ["footprinter_string", "footprint"]
        .iter()
        .find_map(|key| el.get(key).and_then(Value::as_str))
}

fn is_risky(footprint: &str) -> bool {
    let tokens: Vec<&str> = footprint.split('_').collect();
    let is_qfn = tokens[0]
        .strip_prefix("qfn")
        .is_some_and(|n| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()));
    let dim = |prefix: &str| {
        tokens
            .iter()
            .find_map(|t| t.strip_prefix(prefix).and_then(|v| v.parse::<f64>().ok()))
    };
    let sized = tokens
        .iter()
        .any(|t| t.starts_with("pw") || t.starts_with("pl"));
    match (is_qfn, dim("w"), dim("h"), dim("p")) {
        (true, Some(w), Some(h), Some(_)) => {
            !sized && (w < MIN_SAFE_SPAN_MM || h < MIN_SAFE_SPAN_MM)
        }
        _ => false,
    }
}
