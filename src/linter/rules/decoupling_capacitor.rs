//! `decoupling_capacitor`: every IC power pin must have a bypass capacitor.
//!
//! The pins come from `should_have_decoupling_capacitor`. When no pin of the board declares
//! it, the rule says so once at info level and checks every chip power input
//! ([`Role::PowerIn`]) at warning level instead.

use std::collections::HashSet;

use super::bypass_distance::too_far;
use super::erc::parts::PartKind;
use super::erc::roles::Role;
use super::erc::{diagnostic, level, undeclared, Board, Pin, BY_NAME};
use crate::linter::{Category, Diagnostic, LintContext, Rule, RuleMeta, Severity};

/// Verifies that integrated circuits (ICs) have decoupling/bypass capacitors
/// connected to their power supply rails to ensure power integrity.
pub struct DecouplingCapacitorRule;

const META: RuleMeta = RuleMeta {
    id: "decoupling_capacitor",
    category: Category::Erc,
    summary:
        "Each IC power pin needs a bypass capacitor on its rail, placed within 3 mm of the pin.",
    standard: None,
    source_url: None,
    default_severity: Severity::Error,
};

fn needs_cap(pin: &Pin<'_>, declared: bool) -> bool {
    match declared {
        true => pin.attributes().should_have_decoupling_capacitor == Some(true),
        false => pin.kind == PartKind::Chip && pin.role().0 == Role::PowerIn,
    }
}

/// Whether the board declares decoupling, and the pins that need a capacitor.
pub(super) fn power_pins<'b, 'a>(board: &'b Board<'a>) -> (bool, Vec<&'b Pin<'a>>) {
    let declared = board.any(|a| a.should_have_decoupling_capacitor.is_some());
    let pins = board
        .pins
        .iter()
        .filter(|p| p.net.is_some() && needs_cap(p, declared));
    (declared, pins.collect())
}

fn missing_cap(pin: &Pin<'_>, declared: bool) -> Diagnostic {
    let (part, rail) = (pin.refdes(), pin.net_name);
    let name = pin.port.name.as_deref().unwrap_or(pin.id);
    let message = format!(
        "IC \"{part}\" power pin \"{name}\" has no decoupling capacitor on rail \"{rail}\""
    );
    let help = format!(
        "connect a local ceramic bypass capacitor (e.g. 100nF) between \"{rail}\" and GND near \"{part}\""
    );
    diagnostic(
        &META,
        level(declared),
        message,
        &help,
        vec![pin.id.to_string()],
    )
}

impl Rule for DecouplingCapacitorRule {
    fn meta(&self) -> &'static RuleMeta {
        &META
    }

    fn check(&self, ctx: &LintContext<'_>) -> Vec<Diagnostic> {
        let board = Board::new(ctx);
        let (declared, pins) = power_pins(&board);
        let with_caps: HashSet<usize> = board
            .pins
            .iter()
            .filter(|p| p.kind == PartKind::Capacitor)
            .filter_map(|p| p.net)
            .collect();
        let mut reported: HashSet<(&str, usize)> = HashSet::new();
        let missing = pins.iter().filter(|pin| {
            let net = pin.net.unwrap_or(usize::MAX);
            !with_caps.contains(&net) && reported.insert((pin.part_id, net))
        });
        let missing: Vec<Diagnostic> = missing.map(|pin| missing_cap(pin, declared)).collect();
        let fallback = match declared {
            true => None,
            false => undeclared(&META, "`should_have_decoupling_capacitor`", &board, BY_NAME),
        };
        let fallback = fallback.filter(|_| !pins.is_empty());
        fallback
            .into_iter()
            .chain(missing)
            .chain(too_far(&board, &pins))
            .collect()
    }
}
