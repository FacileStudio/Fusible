//! The `rules:` key: a severity per rule id, and the rules no manifest may weaken.

use std::collections::BTreeMap;

use serde::Deserialize;

use super::{ConfigError, FusibleConfig};

/// Rules that guard the netlist itself: they stay errors and can never be waived.
pub const NETLIST_RULES: &[&str] = &[
    "unconnected_net",
    "malformed_element",
    "ghost_net",
    "dangling_reference",
    "shorted_nets",
];

/// The severity a manifest gives one rule, from `off` (not run) up to `error` (blows the fuse).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RuleLevel {
    Off,
    Info,
    Warning,
    Error,
}

/// Whether `rule` is a netlist rule, which can be neither lowered nor waived.
pub fn is_netlist_rule(rule: &str) -> bool {
    NETLIST_RULES.contains(&rule)
}

/// Fails when a netlist rule is set to anything but `error`.
pub(super) fn validate_levels(rules: &BTreeMap<String, RuleLevel>) -> Result<(), String> {
    let lowered = rules
        .iter()
        .find(|(id, level)| is_netlist_rule(id) && **level != RuleLevel::Error);
    match lowered {
        Some((id, _)) => Err(format!(
            "rule `{id}` guards the netlist; it cannot be turned off or lowered below `error`"
        )),
        None => Ok(()),
    }
}

impl FusibleConfig {
    /// Fails when `rules:` or `waivers:` name a rule id that is not in `known`.
    ///
    /// The check pipeline calls this with the ids of the rule registry.
    pub fn validate_rule_ids(&self, known: &[&str]) -> Result<(), ConfigError> {
        let named = self
            .rules
            .keys()
            .map(String::as_str)
            .chain(self.waivers.iter().map(|w| w.rule.as_str()));
        let unknown: Vec<&str> = named.filter(|id| !known.contains(id)).collect();
        match unknown.first() {
            Some(id) => Err(ConfigError::Invalid(format!(
                "unknown rule id `{id}` in `rules:` or `waivers:`; run `fusible rules` for the list"
            ))),
            None => Ok(()),
        }
    }
}
