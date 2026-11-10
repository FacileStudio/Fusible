//! `missing_pullup`: an open-drain net, or a pin that needs an external pull-up or
//! pull-down, without a resistor to the right rail.

use std::collections::BTreeMap;

use super::nets::{is_rail, joined_by};
use super::parts::PartKind;
use super::roles::Role;
use super::{diagnostic, level, undeclared, Board, Pin, BY_NAME};
use crate::linter::{Category, Diagnostic, LintContext, Rule, RuleMeta, Severity};
use crate::parser::PinAttributes;

/// A net that needs a pull resistor and has none. A pin using its internal pull satisfies it.
pub struct MissingPullupRule;

const META: RuleMeta = RuleMeta {
    id: "missing_pullup",
    category: Category::Erc,
    summary: "Open-drain nets and pins that need an external pull-up or pull-down have a resistor to the rail.",
    standard: None,
    source_url: None,
    default_severity: Severity::Error,
};

const I2C_NAMES: &[&str] = &["SDA", "SCL", "SIOC", "SIOD", "I2C"];

fn declares(a: &PinAttributes) -> bool {
    [
        a.is_using_open_drain,
        a.is_using_open_collector,
        a.needs_external_pullup,
        a.needs_external_pulldown,
        a.is_using_internal_pullup,
        a.is_using_internal_pulldown,
        a.is_configured_for_i2c_sda,
        a.is_configured_for_i2c_scl,
    ]
    .iter()
    .any(Option::is_some)
}

/// Whether the pin needs a pull-up (`up`) or a pull-down (`!up`), and whether it already has one.
fn needs(pin: &Pin<'_>, up: bool, declared: bool) -> (bool, bool) {
    let a = pin.attributes();
    let yes = |v: Option<bool>| v == Some(true);
    if !up {
        return (
            yes(a.needs_external_pulldown),
            yes(a.is_using_internal_pulldown),
        );
    }
    let open = Role::declared(a) == Some(Role::OpenDrain);
    let i2c = yes(a.is_configured_for_i2c_sda) || yes(a.is_configured_for_i2c_scl);
    let named = !declared && pin.kind == PartKind::Chip && {
        let name = pin.port.name.as_deref().unwrap_or("").to_ascii_uppercase();
        I2C_NAMES.iter().any(|n| name.contains(n))
    };
    let need = open || i2c || yes(a.needs_external_pullup) || named;
    (need, yes(a.is_using_internal_pullup))
}

fn report(net: &str, pins: &[&Pin<'_>], up: bool, declared: bool) -> Diagnostic {
    let (what, rail) = match up {
        true => ("pull-up", "the supply"),
        false => ("pull-down", "ground"),
    };
    let labels: Vec<String> = pins.iter().map(|p| p.label()).collect();
    let message = format!(
        "net \"{net}\" needs a {what} for {} but no resistor ties it to {rail}",
        labels.join(", ")
    );
    let help = "add a resistor from the net to the rail (e.g. 4.7k for I2C), \
                or set `is_using_internal_pullup` on a pin that enables its own";
    let elements = pins.iter().map(|p| p.id.to_string()).collect();
    diagnostic(&META, level(declared), message, help, elements)
}

/// The report for one net and one direction, when a pin needs a pull and nothing gives it.
fn missing(
    board: &Board<'_>,
    ends: &BTreeMap<usize, Vec<usize>>,
    (net, pins): (usize, &[&Pin<'_>]),
    up: bool,
    declared: bool,
) -> Option<Diagnostic> {
    let states: Vec<(bool, bool)> = pins.iter().map(|p| needs(p, up, declared)).collect();
    let wanting: Vec<&Pin<'_>> = pins
        .iter()
        .zip(&states)
        .filter(|(_, state)| state.0)
        .map(|(pin, _)| *pin)
        .collect();
    let internal = states.iter().any(|state| state.1);
    let pulled = ends
        .get(&net)
        .is_some_and(|others| others.iter().any(|&o| is_rail(board, o, up)));
    let missing = !wanting.is_empty() && !internal && !pulled;
    missing.then(|| report(pins[0].net_name, &wanting, up, declared))
}

impl Rule for MissingPullupRule {
    fn meta(&self) -> &'static RuleMeta {
        &META
    }

    fn check(&self, ctx: &LintContext<'_>) -> Vec<Diagnostic> {
        let board = Board::new(ctx);
        let declared = board.any(declares);
        let ends = joined_by(&board, PartKind::Resistor);
        let mut found: Vec<Diagnostic> = match declared {
            true => Vec::new(),
            false => undeclared(
                &META,
                "open-drain outputs or pull-up needs",
                &board,
                BY_NAME,
            )
            .into_iter()
            .collect(),
        };
        for (net, pins) in board.nets() {
            for up in [true, false] {
                found.extend(missing(&board, &ends, (net, &pins), up, declared));
            }
        }
        found
    }
}
