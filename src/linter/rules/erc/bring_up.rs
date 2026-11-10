//! Bring-up warnings: what the first power-on and the first flash will miss.
//!
//! * `rail_test_point`: a supply rail with no test point or connector pin to probe it on;
//! * `debug_connector`: an MCU with no connector or test point on a debug or programming pin;
//! * `connector_clamp`: a connector pin wired to an MCU GPIO with no clamp diode on the net.
//!
//! The MCU is the chip with the most pins declared `is_gpio`, at least eight.

mod access;

use super::nets::is_rail;
use super::parts::PartKind;
use super::{diagnostic, Board};
use crate::linter::{Category, Diagnostic, LintContext, Rule, RuleMeta, Severity};

/// The three bring-up checks, all warnings.
pub struct BringUpRule;

const fn meta(id: &'static str, summary: &'static str) -> RuleMeta {
    RuleMeta {
        id,
        category: Category::Erc,
        summary,
        standard: None,
        source_url: None,
        default_severity: Severity::Warning,
    }
}

const TEST_POINT: RuleMeta = meta(
    "rail_test_point",
    "Each supply rail has a test point or connector pin to probe it on.",
);
const DEBUG: RuleMeta = meta(
    "debug_connector",
    "The MCU has a connector or test point on a debug or programming pin.",
);
const CLAMP: RuleMeta = meta(
    "connector_clamp",
    "A connector pin wired to an MCU GPIO has a clamp diode on its net.",
);

fn probe(kind: PartKind) -> bool {
    matches!(kind, PartKind::TestPoint | PartKind::Connector)
}

fn test_points(board: &Board<'_>) -> Vec<Diagnostic> {
    let rails = board.nets().filter(|(net, _)| is_rail(board, *net, true));
    rails
        .filter(|(_, pins)| !pins.iter().any(|p| probe(p.kind)))
        .map(|(_, pins)| {
            let name = pins[0].net_name;
            let message = format!("rail \"{name}\" has no test point to probe it on");
            let help = "add a <testpoint /> on the rail so bring-up can measure it";
            let elements = pins.iter().map(|p| p.id.to_string()).collect();
            diagnostic(&TEST_POINT, Severity::Warning, message, help, elements)
        })
        .collect()
}

impl Rule for BringUpRule {
    fn meta(&self) -> &'static RuleMeta {
        &TEST_POINT
    }

    fn emits(&self) -> &'static [&'static RuleMeta] {
        &[&DEBUG, &CLAMP]
    }

    fn check(&self, ctx: &LintContext<'_>) -> Vec<Diagnostic> {
        let board = Board::new(ctx);
        let mut found = test_points(&board);
        found.extend(access::debug(&board, &DEBUG));
        found.extend(access::clamps(&board, &CLAMP));
        found
    }
}
