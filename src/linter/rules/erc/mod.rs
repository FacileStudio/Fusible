//! Electrical rules on pins, rails and parts.
//!
//! Every rule reads pin roles from [`roles`]. Declared attributes report at error level. When
//! no pin of the board declares what a rule needs, the rule says so once at info level and
//! falls back to the name heuristics of [`roles`] at warning level: it never passes silently.

mod board;
mod bring_up;
mod derating;
mod drive;
mod led_series_resistor;
mod missing_pullup;
mod nc_connected;
mod nets;
pub mod parts;
mod pin;
mod pin_conflict;
mod pin_unconnected;
mod power_undriven;
mod rail_budget;
pub mod roles;
mod voltage_domain;

pub use bring_up::BringUpRule;
pub use derating::DeratingRule;
pub use led_series_resistor::LedSeriesResistorRule;
pub use missing_pullup::MissingPullupRule;
pub use nc_connected::NcConnectedRule;
pub use pin_conflict::PinConflictRule;
pub use pin_unconnected::PinUnconnectedRule;
pub use power_undriven::PowerUndrivenRule;
pub use rail_budget::RailBudgetRule;
pub use voltage_domain::VoltageDomainRule;

pub(super) use board::Board;
pub(super) use pin::Pin;

use crate::linter::{Diagnostic, RuleMeta, Severity};

/// Error when the evidence was declared, warning when a name heuristic supplied it.
pub(super) fn level(declared: bool) -> Severity {
    match declared {
        true => Severity::Error,
        false => Severity::Warning,
    }
}

/// A diagnostic of rule `meta`, citing the rule's standard.
pub(super) fn diagnostic(
    meta: &RuleMeta,
    severity: Severity,
    message: String,
    help: &str,
    elements: Vec<String>,
) -> Diagnostic {
    Diagnostic {
        rule: meta.id.to_string(),
        message,
        help: Some(help.to_string()),
        severity,
        standard: meta.standard.map(str::to_string),
        source_url: meta.source_url.map(str::to_string),
        elements,
        ..Default::default()
    }
}

/// `U1, U2, U3, U4 and 3 more`.
fn list(names: &[&str]) -> String {
    match names.len() {
        0..=4 => names.join(", "),
        n => format!("{} and {} more", names[..4].join(", "), n - 4),
    }
}

/// What most rules do when the board declares nothing.
pub(super) const BY_NAME: &str = "it falls back to pin names at warning level";

/// The one info line of a rule whose attributes `attrs` no pin of the board declares, saying
/// what the rule does instead (`then`).
///
/// `None` on a board without chips: there is no part that should have declared them.
pub(super) fn undeclared(
    meta: &RuleMeta,
    attrs: &str,
    board: &Board<'_>,
    then: &str,
) -> Option<Diagnostic> {
    let chips = board.chips();
    if chips.is_empty() {
        return None;
    }
    let message = format!(
        "no pin on {} declares {attrs}, so `{}` cannot rely on it: {then}",
        list(&chips),
        meta.id
    );
    let help = "set `pinAttributes` on the parts in the TSX so the rule reports at error level";
    Some(diagnostic(meta, Severity::Info, message, help, Vec::new()))
}
