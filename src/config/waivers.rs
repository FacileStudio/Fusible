//! The `waivers:` key: reasoned, optionally dated exceptions to one rule at one place.

use serde::Deserialize;

use super::date::Date;
use super::rules::is_netlist_rule;

/// One waived diagnostic: `rule` at the element or `refdes.pin` named by `matches`.
///
/// A waiver needs a `reason`. With `until`, it applies up to and including that day.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Waiver {
    pub rule: String,
    /// Element id or `refdes.pin`, matched exactly. Written `match:` in the manifest.
    #[serde(rename = "match")]
    pub matches: String,
    #[serde(default)]
    pub reason: String,
    #[serde(default)]
    pub until: Option<Date>,
}

impl Waiver {
    /// Whether the waiver still applies on `today`; past its `until` day it is expired.
    pub fn active(&self, today: Date) -> bool {
        self.until.is_none_or(|until| today <= until)
    }
}

/// Fails on a waiver without a rule, a match or a reason, or one that waives a netlist rule.
pub(super) fn validate_waivers(waivers: &[Waiver]) -> Result<(), String> {
    for (index, waiver) in waivers.iter().enumerate() {
        let at = format!("waiver {} (`{}`)", index + 1, waiver.rule);
        if waiver.rule.trim().is_empty() {
            return Err(format!("waiver {} names no rule", index + 1));
        }
        if waiver.matches.trim().is_empty() {
            return Err(format!("{at} has an empty `match`"));
        }
        if waiver.reason.trim().is_empty() {
            return Err(format!("{at} needs a `reason`"));
        }
        if is_netlist_rule(&waiver.rule) {
            return Err(format!(
                "{at} waives a netlist rule; netlist rules cannot be waived"
            ));
        }
    }
    Ok(())
}
