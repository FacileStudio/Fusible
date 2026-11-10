//! `voltage_domain`: a pin's required voltage against its net's declared voltage, and an
//! output of a higher-voltage part driving an input of a lower-voltage one.
//!
//! Runs only when `nets:` declares voltages. A part's domain is the highest voltage its
//! power inputs need: their `requires_voltage`, else the declared voltage of their net.
//! A signal pin that declares `provides_voltage` or `requires_voltage` uses that level
//! in place of its part's domain.

use std::collections::BTreeMap;

use super::roles::Role;
use super::{diagnostic, undeclared, Board, Pin};
use crate::config::DesignRules;
use crate::linter::{Category, Diagnostic, LintContext, Rule, RuleMeta, Severity};

/// Pin voltages against the `nets:` voltages, and drive across domains.
pub struct VoltageDomainRule;

/// Relative tolerance of a pin that declares no `required_voltage_tolerance`.
const DEFAULT_TOLERANCE: f64 = 0.1;

const META: RuleMeta = RuleMeta {
    id: "voltage_domain",
    category: Category::Erc,
    summary:
        "Pins sit on nets of the voltage they need, and no output drives a lower-voltage input.",
    standard: None,
    source_url: None,
    default_severity: Severity::Error,
};

fn tolerance(pin: &Pin<'_>) -> f64 {
    pin.attributes()
        .required_voltage_tolerance
        .unwrap_or(DEFAULT_TOLERANCE)
}

fn net_voltage(board: &Board<'_>, pin: &Pin<'_>) -> Option<f64> {
    board.spec(pin.net?)?.voltage
}

fn mismatch(board: &Board<'_>, pin: &Pin<'_>) -> Option<Diagnostic> {
    let need = pin.attributes().requires_voltage?;
    let have = net_voltage(board, pin)?;
    let tol = tolerance(pin);
    if (have - need).abs() <= need * tol {
        return None;
    }
    let message = format!(
        "{} needs {need} V ±{:.0} % but net \"{}\" is declared at {have} V",
        pin.label(),
        tol * 100.0,
        pin.net_name
    );
    let help =
        "move the pin to a rail of the right voltage, or correct `nets:` or `requires_voltage`";
    Some(diagnostic(
        &META,
        Severity::Error,
        message,
        help,
        vec![pin.id.to_string()],
    ))
}

/// The voltage domain of every part with a powered input.
fn domains<'a>(board: &Board<'a>) -> BTreeMap<&'a str, f64> {
    let mut found: BTreeMap<&str, f64> = BTreeMap::new();
    for pin in board.pins.iter().filter(|p| p.role().0 == Role::PowerIn) {
        let volts = pin
            .attributes()
            .requires_voltage
            .or(net_voltage(board, pin));
        if let Some(v) = volts {
            let domain = found.entry(pin.part_id).or_insert(v);
            *domain = domain.max(v);
        }
    }
    found
}

fn crossing(net: &[&Pin<'_>], domains: &BTreeMap<&str, f64>) -> Option<Diagnostic> {
    let role = |p: &Pin<'_>| Role::declared(p.attributes());
    let drives = |p: &&&Pin<'_>| matches!(role(p), Some(Role::Output | Role::Bidirectional));
    let hears = |p: &&&Pin<'_>| matches!(role(p), Some(Role::Input | Role::Bidirectional));
    let mut pairs = net.iter().filter(drives).flat_map(|d| {
        let others = net.iter().filter(hears).filter(|r| r.part_id != d.part_id);
        others.map(move |r| (*d, *r))
    });
    let (d, r, high, low) = pairs.find_map(|(d, r)| {
        let level = |own: Option<f64>, pin: &Pin<'_>| own.or(domains.get(pin.part_id).copied());
        let high = level(d.attributes().provides_voltage, d)?;
        let low = level(r.attributes().requires_voltage, r)?;
        (high > low * (1.0 + DEFAULT_TOLERANCE)).then_some((d, r, high, low))
    })?;
    let message = format!(
        "{} ({high} V) drives {} ({low} V) on net \"{}\"",
        d.label(),
        r.label(),
        d.net_name
    );
    let help = "add a level shifter or a divider, or power both parts from the same rail";
    Some(diagnostic(
        &META,
        Severity::Error,
        message,
        help,
        vec![d.id.to_string(), r.id.to_string()],
    ))
}

impl Rule for VoltageDomainRule {
    fn meta(&self) -> &'static RuleMeta {
        &META
    }

    fn enabled(&self, design: Option<&DesignRules>) -> bool {
        design.is_some_and(|d| d.nets.values().any(|spec| spec.voltage.is_some()))
    }

    fn check(&self, ctx: &LintContext<'_>) -> Vec<Diagnostic> {
        if !self.enabled(ctx.design) {
            return Vec::new();
        }
        let board = Board::new(ctx);
        let directed = board.any(|a| a.is_input.is_some() || a.is_output.is_some());
        let fallback = match directed {
            true => None,
            false => undeclared(
                &META,
                "`is_input` or `is_output`",
                &board,
                "drive across domains is not checked",
            ),
        };
        let required = board.pins.iter().filter_map(|pin| mismatch(&board, pin));
        let domains = domains(&board);
        let crossings: Vec<Diagnostic> = board
            .nets()
            .filter_map(|(_, pins)| crossing(&pins, &domains))
            .collect();
        fallback
            .into_iter()
            .chain(required)
            .chain(crossings)
            .collect()
    }
}
