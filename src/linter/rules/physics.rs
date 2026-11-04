//! Shared pieces of the rules that check physics and fabrication.

use crate::linter::Diagnostic;
use crate::parser::geometry::Geometry;

/// Closing sentence of every physics help text.
pub(super) const PRECHECK: &str = "Design pre-check, not certification.";

/// Whether `layer` is an outer copper layer.
pub(super) fn is_external(layer: &str) -> bool {
    matches!(layer, "top" | "bottom")
}

/// One error naming the PCB elements the reader could not parse, or `None`
/// when all of them were readable. A physical rule cannot verify what it
/// cannot read, so it reports it.
pub(super) fn unreadable(rule: &str, geometry: &Geometry) -> Option<Diagnostic> {
    if geometry.unreadable.is_empty() {
        return None;
    }
    let count = geometry.unreadable.len();
    Some(Diagnostic {
        rule: rule.to_string(),
        message: format!(
            "{count} PCB element(s) cannot be verified: {}",
            geometry.unreadable.join("; ")
        ),
        help: Some(format!(
            "give each element its position, width and diameters so the check can read it. {PRECHECK}"
        )),
        ..Default::default()
    })
}

/// A length in millimetres with at least two and at most three decimals.
pub(super) fn mm(value: f64) -> String {
    let text = format!("{value:.3}");
    match text.ends_with('0') {
        true => text[..text.len() - 1].to_string(),
        false => text,
    }
}
