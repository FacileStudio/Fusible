//! `pin_unconnected`: a pin that must be connected sits on no net, or alone on its net.

use super::parts::PartKind;
use super::roles::Role;
use super::{diagnostic, level, undeclared, Board, Pin, BY_NAME};
use crate::linter::{Category, Diagnostic, LintContext, Rule, RuleMeta, Severity};
use crate::parser::PinAttributes;

/// A `must_be_connected`, power or ground pin with nothing else on its net.
pub struct PinUnconnectedRule;

const META: RuleMeta = RuleMeta {
    id: "pin_unconnected",
    category: Category::Erc,
    summary: "Every pin declared `must_be_connected`, and every power or ground input, is wired.",
    standard: None,
    source_url: None,
    default_severity: Severity::Error,
};

fn declares(a: &PinAttributes) -> bool {
    a.must_be_connected.is_some() || a.requires_power.is_some() || a.requires_ground.is_some()
}

fn required(pin: &Pin<'_>, declared: bool) -> bool {
    let supply = |role| matches!(role, Some(Role::PowerIn) | Some(Role::GroundIn));
    match declared {
        true => {
            pin.attributes().must_be_connected == Some(true)
                || supply(Role::declared(pin.attributes()))
        }
        false => pin.kind == PartKind::Chip && supply(Some(pin.guessed())),
    }
}

fn alone(board: &Board<'_>, pin: &Pin<'_>) -> bool {
    pin.net.is_none_or(|net| board.endpoints(net) < 2)
}

fn report(pin: &Pin<'_>, declared: bool) -> Diagnostic {
    let message = format!(
        "{} must be connected but nothing else is on its net",
        pin.label()
    );
    let help = "wire the pin, or drop `must_be_connected` if the part allows it to float";
    diagnostic(
        &META,
        level(declared),
        message,
        help,
        vec![pin.id.to_string()],
    )
}

impl Rule for PinUnconnectedRule {
    fn meta(&self) -> &'static RuleMeta {
        &META
    }

    fn check(&self, ctx: &LintContext<'_>) -> Vec<Diagnostic> {
        let board = Board::new(ctx);
        let declared = board.any(declares);
        let fallback = match declared {
            true => None,
            false => undeclared(&META, "`must_be_connected`", &board, BY_NAME),
        };
        let found = board
            .pins
            .iter()
            .filter(|pin| required(pin, declared) && alone(&board, pin))
            .map(|pin| report(pin, declared));
        fallback.into_iter().chain(found).collect()
    }
}
