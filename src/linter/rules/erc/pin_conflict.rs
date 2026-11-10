//! `pin_conflict`: two pins on one net that fight each other, two drivers above all.

use super::roles::{declares_role, Role};
use super::{diagnostic, level, undeclared, Board, Pin, BY_NAME};
use crate::linter::{Category, Diagnostic, LintContext, Rule, RuleMeta, Severity};

/// One net with conflicting pins of two different parts: the worst pair found.
pub struct PinConflictRule;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Cell {
    Ok,
    Warn,
    Err,
}

use Cell::{Err as E, Ok as O, Warn as W};

/// Rows and columns follow [`Role`]: power in, power out, ground in, ground out, push-pull
/// output, open-drain output, input, bidirectional, passive, no-connect, unspecified.
///
/// Cells between outputs, open-drain outputs, power outputs, bidirectional and inputs are
/// KiCad's default pin conflicts map (`ERC_SETTINGS::m_defaultPinMap`, `eeschema/erc/erc_settings.cpp`
/// at KiCad 10.0.3): output against output, power output or open collector is an error,
/// two power outputs are an error, power output against bidirectional is a warning. Ground
/// pins follow the power pins with two Fusible additions: grounds tie together freely, and a
/// supply pin on a ground net is an error. No-connect and unspecified pins are `Ok` here:
/// `nc_connected` and the role fallback report them.
const MATRIX: [[Cell; 11]; 11] = [
    [O, O, E, E, O, O, O, O, O, O, O],
    [O, E, E, E, E, E, O, W, O, O, O],
    [E, E, O, O, O, O, O, O, O, O, O],
    [E, E, O, O, E, E, O, W, O, O, O],
    [O, E, O, E, E, E, O, O, O, O, O],
    [O, E, O, E, E, O, O, O, O, O, O],
    [O, O, O, O, O, O, O, O, O, O, O],
    [O, W, O, W, O, O, O, O, O, O, O],
    [O, O, O, O, O, O, O, O, O, O, O],
    [O, O, O, O, O, O, O, O, O, O, O],
    [O, O, O, O, O, O, O, O, O, O, O],
];

const META: RuleMeta = RuleMeta {
    id: "pin_conflict",
    category: Category::Erc,
    summary: "No net joins two pins that drive against each other, such as two push-pull outputs.",
    standard: Some("KiCad ERC pin conflicts map"),
    source_url: Some(
        "https://gitlab.com/kicad/code/kicad/-/blob/10.0.3/eeschema/erc/erc_settings.cpp",
    ),
    default_severity: Severity::Error,
};

type Conflict<'p, 'a> = (Cell, bool, &'p Pin<'a>, Role, &'p Pin<'a>, Role);

fn worst<'p, 'a>(pins: &[&'p Pin<'a>]) -> Option<Conflict<'p, 'a>> {
    let roles: Vec<(Role, bool)> = pins.iter().map(|p| p.role()).collect();
    let mut found: Option<Conflict<'p, 'a>> = None;
    for i in 0..pins.len() {
        for j in i + 1..pins.len() {
            let ((a, da), (b, db)) = (roles[i], roles[j]);
            let cell = MATRIX[a as usize][b as usize];
            let declared = da && db;
            let better = found.is_none_or(|f| (cell, declared) > (f.0, f.1));
            let apart = pins[i].part_id != pins[j].part_id;
            if cell != Cell::Ok && better && apart {
                found = Some((cell, declared, pins[i], a, pins[j], b));
            }
        }
    }
    found
}

fn report(net: &str, conflict: Conflict<'_, '_>) -> Diagnostic {
    let (cell, declared, a, ra, b, rb) = conflict;
    let severity = match cell {
        Cell::Err => level(declared),
        _ => Severity::Warning,
    };
    let message = format!(
        "net \"{net}\" joins {} ({}) and {} ({})",
        a.label(),
        ra.label(),
        b.label(),
        rb.label()
    );
    let help =
        "two drivers fight over one net: remove one, make it open-drain, or fix the pin role";
    let elements = vec![a.id.to_string(), b.id.to_string()];
    diagnostic(&META, severity, message, help, elements)
}

impl Rule for PinConflictRule {
    fn meta(&self) -> &'static RuleMeta {
        &META
    }

    fn check(&self, ctx: &LintContext<'_>) -> Vec<Diagnostic> {
        let board = Board::new(ctx);
        let fallback = match board.any(declares_role) {
            true => None,
            false => undeclared(&META, "a role", &board, BY_NAME),
        };
        let conflicts = board.nets().filter_map(|(_, pins)| {
            let conflict = worst(&pins)?;
            Some(report(pins[0].net_name, conflict))
        });
        fallback.into_iter().chain(conflicts).collect()
    }
}
