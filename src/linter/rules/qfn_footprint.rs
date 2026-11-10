//! `qfn_footprint_span`: a bare qfn footprint string describes copper span, not body size.

use crate::linter::{Category, Diagnostic, LintContext, Rule, RuleMeta, Severity};

const MIN_SAFE_SPAN_MM: f64 = 3.5;
const MESSAGE: &str = "qfn w/h is the copper span, not the body; corner pads may short";
const HELP: &str = "set explicit pad size, e.g. qfn16_w3.8_h3.8_p0.5_pw0.25_pl0.8";

/// Warns on `qfn<N>_w<W>_h<H>_p<P>` footprints with no `_pw`/`_pl` pad size and a small span.
pub struct QfnFootprintSpanRule;

const META: RuleMeta = RuleMeta {
    id: "qfn_footprint_span",
    category: Category::Dfm,
    summary:
        "Warns when a qfn footprint gives only a copper span, which can short the corner pads.",
    standard: Some("tscircuit footprinter"),
    source_url: None,
    default_severity: Severity::Warning,
};

impl Rule for QfnFootprintSpanRule {
    fn meta(&self) -> &'static RuleMeta {
        &META
    }

    fn check(&self, ctx: &LintContext<'_>) -> Vec<Diagnostic> {
        let mut seen: Vec<&str> = Vec::new();
        for footprint in ctx.circuit.model().footprints.iter().map(String::as_str) {
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
