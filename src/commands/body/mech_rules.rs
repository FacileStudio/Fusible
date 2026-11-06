//! The mechanical rules: process-aware design checks on the features a part recorded.
//!
//! Each rule reads recorded features only, so a unit test needs no kernel. Every message ends
//! with a disclaimer and every finding names the guideline it comes from.

use super::mech_rules_data::*;
use super::mech_rules_parts::{boss, port_clearance, standoff_height, vent_width};
use super::part_features::Feature;
use crate::linter::{Diagnostic, Severity};

pub const WALL_THICKNESS: &str = "mech.wall_thickness";
pub const BOSS: &str = "mech.boss";
pub const PORT_CLEARANCE: &str = "mech.port_clearance";
pub const VENT_WIDTH: &str = "mech.vent_width";
pub const STANDOFF_HEIGHT: &str = "mech.standoff_height";
pub const COMPONENT_CLEARANCE: &str = "mech.component_clearance";
pub const DRAFT: &str = "mech.draft";

/// One broken guideline of one part.
#[derive(Debug, Clone, PartialEq)]
pub struct Finding {
    pub rule: &'static str,
    pub severity: Severity,
    pub message: String,
    pub help: String,
    pub source_url: &'static str,
}

impl Finding {
    pub(super) fn new(
        rule: &'static str,
        severity: Severity,
        text: String,
        help: &str,
        source_url: &'static str,
    ) -> Self {
        Self {
            rule,
            severity,
            message: format!("{text} {DISCLAIMER}"),
            help: help.to_string(),
            source_url,
        }
    }

    pub(super) fn error(rule: &'static str, text: String, help: &str, url: &'static str) -> Self {
        Self::new(rule, Severity::Error, text, help, url)
    }

    /// The linter diagnostic of this finding, with its guideline source.
    pub fn diagnostic(&self) -> Diagnostic {
        Diagnostic {
            rule: self.rule.to_string(),
            message: self.message.clone(),
            help: Some(self.help.clone()),
            severity: self.severity,
            standard: Some("design guideline".to_string()),
            source_url: Some(self.source_url.to_string()),
            ..Default::default()
        }
    }
}

fn wall_thickness(part: &str, process: &str, features: &[Feature]) -> Vec<Finding> {
    let min = wall_min(process);
    let walls = features.iter().filter_map(|f| match f {
        Feature::Wall { thickness, part } => Some((*thickness, part.as_deref().unwrap_or("wall"))),
        _ => None,
    });
    let found = walls.flat_map(|(t, what)| {
        let thin = (t < min).then(|| {
            let text = format!("`{part}` has a {t} mm {what}; {process} needs at least {min} mm.");
            Finding::error(WALL_THICKNESS, text, "thicken the wall", DFAM)
        });
        let sink = (process == "injection" && t > INJECTION_WALL_MAX).then(|| {
            let text =
                format!("`{part}` has a {t} mm {what}; thick walls sink in injection molding.");
            Finding::error(
                WALL_THICKNESS,
                text,
                "keep molded walls under 5 mm or core them out",
                RTP,
            )
        });
        thin.into_iter().chain(sink)
    });
    found.collect()
}

fn draft(part: &str, process: &str, features: &[Feature]) -> Vec<Finding> {
    let drafted = features
        .iter()
        .any(|f| matches!(f, Feature::Draft { deg } if *deg >= MIN_DRAFT_DEG));
    if process != "injection" || drafted {
        return Vec::new();
    }
    let text =
        format!("`{part}` is molded but declares no draft of at least {MIN_DRAFT_DEG} degrees.");
    vec![Finding::error(
        DRAFT,
        text,
        "declare cad.draft(deg) for molded parts",
        RTP,
    )]
}

fn component_clearance(part: &str, process: &str, features: &[Feature]) -> Vec<Finding> {
    let min = clearance_gap(process);
    features
        .iter()
        .filter_map(|f| match f {
            Feature::Interference { component, volume, gap } => {
                let text = format!(
                    "`{part}` comes within {gap} mm of `{component}` ({volume:.2} mm3 inside that gap)."
                );
                Some(Finding::error(COMPONENT_CLEARANCE, text, "keep a gap around internal parts", HUBS))
            }
            Feature::Clearance { gap } if *gap < min => {
                let text = format!("`{part}` leaves {gap} mm around the board; {process} needs {min} mm.");
                Some(Finding::error(COMPONENT_CLEARANCE, text, "widen the gap around the board", HUBS))
            }
            _ => None,
        })
        .collect()
}

/// Runs every mechanical rule on the `features` of `part`, built with `process`.
pub fn check(part: &str, process: &str, features: &[Feature]) -> Vec<Finding> {
    let mut found = wall_thickness(part, process, features);
    found.extend(boss(part, process, features));
    found.extend(port_clearance(part, process, features));
    found.extend(vent_width(part, process, features));
    found.extend(standoff_height(part, features));
    found.extend(component_clearance(part, process, features));
    found.extend(draft(part, process, features));
    found
}
