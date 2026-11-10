//! `led_series_resistor`: an LED with one leg on a supply rail and no resistor on the other.

use super::nets::{is_rail, joined_by};
use super::parts::PartKind;
use super::{diagnostic, Board, Pin};
use crate::linter::{Category, Diagnostic, LintContext, Rule, RuleMeta, Severity};

/// An LED with one leg on a supply rail and the other on ground, on another rail, or on a
/// net no resistor reaches: nothing limits its current.
pub struct LedSeriesResistorRule;

const META: RuleMeta = RuleMeta {
    id: "led_series_resistor",
    category: Category::Erc,
    summary: "An LED on a supply rail has a series resistor to limit its current.",
    standard: None,
    source_url: None,
    default_severity: Severity::Warning,
};

fn report(led: &Pin<'_>, rail: &str) -> Diagnostic {
    let message = format!(
        "{} sits on rail \"{rail}\" with no series resistor to limit its current",
        led.refdes()
    );
    let help = "add a resistor in series, sized (V_rail - V_f) / I_led";
    diagnostic(
        &META,
        Severity::Warning,
        message,
        help,
        vec![led.part_id.to_string()],
    )
}

impl Rule for LedSeriesResistorRule {
    fn meta(&self) -> &'static RuleMeta {
        &META
    }

    fn check(&self, ctx: &LintContext<'_>) -> Vec<Diagnostic> {
        let board = Board::new(ctx);
        let resistors = joined_by(&board, PartKind::Resistor);
        let limited = |net: usize| resistors.contains_key(&net);
        let mut found = Vec::new();
        let leds = board.pins.iter().filter(|p| p.kind == PartKind::Led);
        let mut seen: Vec<&str> = Vec::new();
        for pin in leds {
            let other = board
                .pins
                .iter()
                .find(|p| p.part_id == pin.part_id && p.id != pin.id);
            let (Some(net), Some(other)) = (pin.net, other) else {
                continue;
            };
            let across = |o: usize| is_rail(&board, o, false) || is_rail(&board, o, true);
            let free = other
                .net
                .is_some_and(|o| o != net && (across(o) || !limited(o)));
            if is_rail(&board, net, true) && free && !seen.contains(&pin.part_id) {
                seen.push(pin.part_id);
                found.push(report(pin, pin.net_name));
            }
        }
        found
    }
}
