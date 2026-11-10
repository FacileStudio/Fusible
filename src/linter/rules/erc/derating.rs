//! `derating`: a capacitor rated too close to the voltage of the rail it sits on.
//!
//! The factor lives in `data/derating.yml` with its source. Runs only when `nets:` declares
//! voltages; a capacitor on a declared rail without `max_voltage_rating` is listed once at
//! info level.

use std::sync::OnceLock;

use serde::Deserialize;

use super::parts::PartKind;
use super::{diagnostic, Board, Pin};
use crate::config::DesignRules;
use crate::linter::{Category, Diagnostic, LintContext, Rule, RuleMeta, Severity};

/// A capacitor whose `max_voltage_rating` is under the factor times its rail voltage.
pub struct DeratingRule;

const DATA: &str = include_str!("../data/derating.yml");

#[derive(Debug, Deserialize)]
struct Derating {
    source: String,
    capacitor_voltage_factor: f64,
}

fn data() -> &'static Derating {
    static LOADED: OnceLock<Derating> = OnceLock::new();
    LOADED.get_or_init(|| serde_yaml::from_str(DATA).expect("data/derating.yml is valid"))
}

const META: RuleMeta = RuleMeta {
    id: "derating",
    category: Category::Erc,
    summary: "Each capacitor is rated for at least 1.25 times the voltage of its rail.",
    standard: Some("voltage derating guideline"),
    source_url: Some(
        "https://industrialmonitordirect.com/blogs/knowledgebase/electrolytic-capacitor-voltage-rating-selection-guide",
    ),
    default_severity: Severity::Error,
};

/// The highest declared voltage among the nets of the capacitor's pins.
fn rail(board: &Board<'_>, pins: &[&Pin<'_>]) -> Option<f64> {
    let volts = pins.iter().filter_map(|p| board.spec(p.net?)?.voltage);
    volts.fold(None, |max, v| Some(max.map_or(v, |m: f64| m.max(v))))
}

fn underrated(cap: &Pin<'_>, rating: f64, volts: f64) -> Option<Diagnostic> {
    let factor = data().capacitor_voltage_factor;
    if rating >= factor * volts {
        return None;
    }
    let message = format!(
        "{} is rated {rating} V on a {volts} V rail; derating asks for at least {:.2} V",
        cap.refdes(),
        factor * volts
    );
    let help = format!(
        "pick a capacitor rated for {:.0} V or more. Source: {}",
        (factor * volts).ceil(),
        data().source
    );
    Some(diagnostic(
        &META,
        Severity::Error,
        message,
        &help,
        vec![cap.part_id.to_string()],
    ))
}

fn unrated(names: &[&str]) -> Diagnostic {
    let message = format!(
        "{} sit on declared rails without a `max_voltage_rating`, so `derating` cannot check them",
        names.join(", ")
    );
    let help = "set `maxVoltageRating` on the capacitors in the TSX";
    diagnostic(&META, Severity::Info, message, help, Vec::new())
}

impl Rule for DeratingRule {
    fn meta(&self) -> &'static RuleMeta {
        &META
    }

    fn enabled(&self, design: Option<&DesignRules>) -> bool {
        design.is_some_and(|d| d.nets.values().any(|spec| spec.voltage.is_some()))
    }

    fn check(&self, ctx: &LintContext<'_>) -> Vec<Diagnostic> {
        let board = Board::new(ctx);
        let caps = board.pins.iter().filter(|p| p.kind == PartKind::Capacitor);
        let mut parts: Vec<(&str, Vec<&Pin<'_>>)> = Vec::new();
        for pin in caps {
            match parts.iter_mut().find(|(id, _)| *id == pin.part_id) {
                Some((_, pins)) => pins.push(pin),
                None => parts.push((pin.part_id, vec![pin])),
            }
        }
        let mut found = Vec::new();
        let mut missing = Vec::new();
        for (_, pins) in &parts {
            let Some(volts) = rail(&board, pins) else {
                continue;
            };
            match pins[0].part.and_then(|c| c.props.max_voltage_rating) {
                Some(rating) => found.extend(underrated(pins[0], rating, volts)),
                None => missing.push(pins[0].refdes()),
            }
        }
        found.extend((!missing.is_empty()).then(|| unrated(&missing)));
        found
    }
}
