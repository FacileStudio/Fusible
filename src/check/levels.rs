//! The manifest's `rules:` levels and `waivers:`, applied to the linter's diagnostics.

use std::collections::BTreeMap;

use super::report::{WaivedDiagnostic, WaiverEntry};
use super::waive::matches;
use super::CheckError;
use crate::config::{Date, FusibleConfig, RuleLevel, Waiver};
use crate::linter::{Diagnostic, Linter, Severity};
use crate::parser::Circuit;

/// Rule id of the `links:` check, outside the linter's registry.
const LINK_RULE: &str = "harness_mismatch";

/// Rule ids of the assembly and body checks, outside the linter's registry.
const ASSEMBLY_RULES: &[&str] = &[
    "body_interference",
    "hole_misaligned",
    "connector_blocked",
    "mech.wall_thickness",
    "mech.boss",
    "mech.port_clearance",
    "mech.vent_width",
    "mech.standoff_height",
    "mech.component_clearance",
    "mech.draft",
    "mech.stale",
    "mech.not_built",
];

/// Every rule id a manifest may name: the linter registry plus the cross-world rules.
pub fn known_rule_ids() -> Vec<&'static str> {
    let registry = Linter::default().describe(None).into_iter().map(|r| r.id);
    let cross = std::iter::once(LINK_RULE).chain(ASSEMBLY_RULES.iter().copied());
    registry.chain(cross).collect()
}

/// The levels and waivers of one check run, with the waivers that matched so far.
pub(super) struct Levels<'a> {
    rules: &'a BTreeMap<String, RuleLevel>,
    active: Vec<(&'a Waiver, bool)>,
    expired: Vec<&'a Waiver>,
}

impl<'a> Levels<'a> {
    /// Validates the rule ids of `config` and splits its waivers into active and expired on `today`.
    pub(super) fn new(config: &'a FusibleConfig, today: Date) -> Result<Self, CheckError> {
        config.validate_rule_ids(&known_rule_ids())?;
        let (active, expired): (Vec<&Waiver>, Vec<&Waiver>) =
            config.waivers.iter().partition(|w| w.active(today));
        Ok(Self {
            rules: &config.rules,
            active: active.into_iter().map(|w| (w, false)).collect(),
            expired,
        })
    }

    /// Sets each diagnostic's level, drops the `off` ones and moves the waived ones to `waived`.
    pub(super) fn apply(
        &mut self,
        diagnostics: Vec<Diagnostic>,
        circuit: Option<&Circuit>,
        board: Option<&str>,
        waived: &mut Vec<WaivedDiagnostic>,
    ) -> Vec<Diagnostic> {
        let mut kept = Vec::new();
        for mut diagnostic in diagnostics {
            let Some(severity) = self.level(&diagnostic) else {
                continue;
            };
            diagnostic.severity = severity;
            match self.waiver_for(&diagnostic, circuit) {
                Some(waiver) => waived.push(WaivedDiagnostic {
                    board: board.map(str::to_string),
                    diagnostic,
                    reason: waiver.reason.clone(),
                    until: waiver.until.map(|d| d.to_string()),
                }),
                None => kept.push(diagnostic),
            }
        }
        kept
    }

    fn level(&self, diagnostic: &Diagnostic) -> Option<Severity> {
        match self.rules.get(&diagnostic.rule) {
            None => Some(diagnostic.severity),
            Some(RuleLevel::Off) => None,
            Some(RuleLevel::Info) => Some(Severity::Info),
            Some(RuleLevel::Warning) => Some(Severity::Warning),
            Some(RuleLevel::Error) => Some(Severity::Error),
        }
    }

    fn waiver_for(
        &mut self,
        diagnostic: &Diagnostic,
        circuit: Option<&Circuit>,
    ) -> Option<&'a Waiver> {
        let (waiver, used) = self
            .active
            .iter_mut()
            .find(|(w, _)| matches(w, diagnostic, circuit))?;
        *used = true;
        Some(*waiver)
    }

    /// One `unused_waiver` warning per active waiver that matched no diagnostic.
    ///
    /// Without `assembly_ran`, waivers of assembly and body rules had no chance to match
    /// and are left out.
    pub(super) fn unused(&self, assembly_ran: bool) -> Vec<Diagnostic> {
        let unused = self.active.iter().filter(|(w, used)| {
            !used && (assembly_ran || !ASSEMBLY_RULES.contains(&w.rule.as_str()))
        });
        unused.map(|(w, _)| unused_warning(w)).collect()
    }

    /// The waivers past their `until` day.
    pub(super) fn expired(&self) -> Vec<WaiverEntry> {
        self.expired.iter().map(|w| WaiverEntry::from(*w)).collect()
    }
}

fn unused_warning(waiver: &Waiver) -> Diagnostic {
    Diagnostic {
        rule: "unused_waiver".into(),
        message: format!(
            "the waiver for `{}` at `{}` matches no diagnostic",
            waiver.rule, waiver.matches
        ),
        help: Some(
            "remove it from `waivers:` if the board was fixed, or correct its `match`".into(),
        ),
        severity: Severity::Warning,
        elements: vec![waiver.matches.clone()],
        ..Default::default()
    }
}
