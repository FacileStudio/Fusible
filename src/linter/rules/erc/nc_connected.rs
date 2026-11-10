//! `nc_connected`: a `do_not_connect` pin shares its net with another endpoint.

use std::collections::HashSet;

use super::parts::PartKind;
use super::roles::{is_no_connect_name, Role};
use super::{diagnostic, level, undeclared, Board, Pin, BY_NAME};
use crate::linter::{Category, Diagnostic, LintContext, Rule, RuleMeta, Severity};

/// A pin that must stay open but is wired.
///
/// A pin the part ties internally to a wired pin (`source_component_internal_connection`,
/// e.g. the second leg pair of a four-pin push button) shares that pin's net without being
/// wired itself, so it is reported only when a trace or a net names it directly.
pub struct NcConnectedRule;

const META: RuleMeta = RuleMeta {
    id: "nc_connected",
    category: Category::Erc,
    summary: "A pin declared `do_not_connect` shares its net with no other endpoint.",
    standard: Some("KiCad ERC pin conflicts map (no-connect row)"),
    source_url: Some(
        "https://gitlab.com/kicad/code/kicad/-/blob/10.0.3/eeschema/erc/erc_settings.cpp",
    ),
    default_severity: Severity::Error,
};

fn is_candidate(pin: &Pin<'_>, declared: bool) -> bool {
    match declared {
        true => Role::declared(pin.attributes()) == Some(Role::NoConnect),
        false => {
            let name = pin.port.name.as_deref().unwrap_or("");
            pin.kind == PartKind::Chip && is_no_connect_name(name)
        }
    }
}

fn wired(board: &Board<'_>, pin: &Pin<'_>, internal: &HashSet<&str>) -> bool {
    if internal.contains(pin.id) {
        let traced = board
            .ctx
            .circuit
            .traces()
            .any(|t| t.connected_source_port_ids.iter().any(|id| id == pin.id));
        return traced || pin.port.net_id.is_some();
    }
    pin.net.is_some_and(|net| board.endpoints(net) >= 2)
}

fn report(pin: &Pin<'_>, declared: bool) -> Diagnostic {
    let message = format!(
        "{} must stay unconnected but is wired to net \"{}\"",
        pin.label(),
        pin.net_name
    );
    let help = "remove the connection, or drop `do_not_connect` if the datasheet allows it";
    diagnostic(
        &META,
        level(declared),
        message,
        help,
        vec![pin.id.to_string()],
    )
}

impl Rule for NcConnectedRule {
    fn meta(&self) -> &'static RuleMeta {
        &META
    }

    fn check(&self, ctx: &LintContext<'_>) -> Vec<Diagnostic> {
        let board = Board::new(ctx);
        let declared = board.any(|a| a.do_not_connect.is_some());
        let internal: HashSet<&str> = ctx
            .circuit
            .model()
            .internal_connections
            .iter()
            .flat_map(|c| c.source_port_ids.iter().map(String::as_str))
            .collect();
        let fallback = match declared {
            true => None,
            false => undeclared(&META, "`do_not_connect`", &board, BY_NAME),
        };
        let found = board
            .pins
            .iter()
            .filter(|pin| is_candidate(pin, declared) && wired(&board, pin, &internal))
            .map(|pin| report(pin, declared));
        fallback.into_iter().chain(found).collect()
    }
}
