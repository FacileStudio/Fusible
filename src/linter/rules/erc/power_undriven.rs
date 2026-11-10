//! `power_undriven`: a net with a power input and nothing that supplies it.

use std::collections::BTreeMap;

use super::drive::supplied;
use super::parts::PartKind;
use super::roles::Role;
use super::{diagnostic, level, undeclared, Board, Pin, BY_NAME};
use crate::linter::{Category, Diagnostic, LintContext, Rule, RuleMeta, Severity};

/// A power input must sit on a net that a power output, a connector, a battery, a source,
/// a series part from a supplied net, or a `nets:` entry with `source: external` supplies.
pub struct PowerUndrivenRule;

const META: RuleMeta = RuleMeta {
    id: "power_undriven",
    category: Category::Erc,
    summary: "Every power input sits on a net that something supplies.",
    standard: Some("KiCad ERC power pin not driven"),
    source_url: Some("https://gitlab.com/kicad/code/kicad/-/blob/10.0.3/eeschema/erc/erc.cpp"),
    default_severity: Severity::Error,
};

/// What the board declares: power inputs and power outputs.
struct Declared {
    inputs: bool,
    outputs: bool,
}

fn is_consumer(pin: &Pin<'_>, declared: &Declared) -> bool {
    match declared.inputs {
        true => Role::declared(pin.attributes()) == Some(Role::PowerIn),
        false => pin.kind == PartKind::Chip && pin.guessed() == Role::PowerIn,
    }
}

fn drives(pin: &Pin<'_>, declared: &Declared) -> bool {
    let named = !declared.outputs && pin.kind == PartKind::Chip && pin.guessed() == Role::PowerOut;
    pin.kind.brings_power() || Role::declared(pin.attributes()) == Some(Role::PowerOut) || named
}

fn report(net: &str, pins: &[&Pin<'_>], declared: &Declared) -> Diagnostic {
    let labels: Vec<String> = pins.iter().map(|p| p.label()).collect();
    let message = format!(
        "net \"{net}\" feeds {} but nothing supplies it",
        labels.join(", ")
    );
    let help = "add a regulator or connector on it, declare `provides_power` on its source, \
                or set `source: external` for the net under `nets:` in fusible.yml";
    let elements = pins.iter().map(|p| p.id.to_string()).collect();
    let severity = level(declared.inputs && declared.outputs);
    diagnostic(&META, severity, message, help, elements)
}

fn fallback(board: &Board<'_>, declared: &Declared) -> Option<Diagnostic> {
    let missing: Vec<&str> = [
        (declared.inputs, "`requires_power`"),
        (declared.outputs, "`provides_power`"),
    ]
    .into_iter()
    .filter(|(set, _)| !set)
    .map(|(_, name)| name)
    .collect();
    match missing.is_empty() {
        true => None,
        false => undeclared(&META, &missing.join(" or "), board, BY_NAME),
    }
}

impl Rule for PowerUndrivenRule {
    fn meta(&self) -> &'static RuleMeta {
        &META
    }

    fn check(&self, ctx: &LintContext<'_>) -> Vec<Diagnostic> {
        let board = Board::new(ctx);
        let declared = Declared {
            inputs: board.any(|a| a.requires_power.is_some()),
            outputs: board.any(|a| a.provides_power.is_some()),
        };
        let supplied = supplied(&board, |pin| drives(pin, &declared));
        let mut starved: BTreeMap<usize, Vec<&Pin<'_>>> = BTreeMap::new();
        for pin in board.pins.iter().filter(|p| is_consumer(p, &declared)) {
            if let Some(net) = pin.net.filter(|net| !supplied.contains(net)) {
                starved.entry(net).or_default().push(pin);
            }
        }
        let found = starved
            .values()
            .map(|pins| report(pins[0].net_name, pins, &declared));
        fallback(&board, &declared)
            .into_iter()
            .chain(found)
            .collect()
    }
}
