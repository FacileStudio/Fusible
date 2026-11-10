//! Reaching the board from outside: the MCU's debug access and the clamps on connector pins.

use super::super::nets::is_rail;
use super::super::parts::PartKind;
use super::super::{diagnostic, Board, Pin};
use super::probe;
use crate::linter::{Diagnostic, RuleMeta, Severity};

/// Name tokens of debug and programming signals: SWD, JTAG, reset, boot, UART, USB.
const DEBUG_TOKENS: &[&str] = &[
    "SWDIO", "SWCLK", "SWO", "TCK", "TMS", "TDI", "TDO", "NRST", "RESET", "RST", "BOOT", "TX",
    "RX", "TXD", "RXD", "DP", "DM", "D+", "D-",
];

/// Whether a pin or net name is a debug or programming signal.
fn is_debug_name(name: &str) -> bool {
    let upper = name.to_ascii_uppercase();
    let prefixed = ["USB", "UART", "SWD", "JTAG", "D_POS", "D_NEG"];
    let tokens = upper.split(|c: char| !(c.is_ascii_alphanumeric() || c == '+' || c == '-'));
    prefixed.iter().any(|p| upper.contains(p))
        || tokens.into_iter().any(|t| DEBUG_TOKENS.contains(&t))
}

fn is_gpio(pin: &Pin<'_>) -> bool {
    pin.attributes().is_gpio == Some(true)
}

/// The chip with the most GPIO pins, when it has at least eight.
fn mcu<'a>(board: &Board<'a>) -> Option<&'a str> {
    let chips = board.chips();
    let count = |name: &str| {
        let gpio = board
            .pins
            .iter()
            .filter(|p| p.refdes() == name && is_gpio(p));
        gpio.count()
    };
    let best = chips.into_iter().max_by_key(|name| count(name))?;
    (count(best) >= 8).then_some(best)
}

/// A warning when the MCU has no connector or test point on a debug or programming signal.
pub(super) fn debug(board: &Board<'_>, meta: &RuleMeta) -> Option<Diagnostic> {
    let mcu = mcu(board)?;
    let reachable = board.pins.iter().filter(|p| probe(p.kind)).any(|p| {
        let names = [p.port.name.as_deref().unwrap_or(""), p.net_name];
        names.iter().any(|n| is_debug_name(n))
            || p.net.is_some_and(|net| {
                board.on_net(net).any(|q| {
                    q.refdes() == mcu && is_debug_name(q.port.name.as_deref().unwrap_or(""))
                })
            })
    });
    let message = format!("{mcu} has no debug or programming connector");
    let help = "bring SWD, JTAG, UART or USB of the MCU to a connector or test points";
    (!reachable).then(|| {
        diagnostic(
            meta,
            Severity::Warning,
            message,
            help,
            vec![mcu.to_string()],
        )
    })
}

/// One warning per connector pin wired to an MCU GPIO with no diode on the net.
pub(super) fn clamps(board: &Board<'_>, meta: &RuleMeta) -> Vec<Diagnostic> {
    let Some(mcu) = mcu(board) else {
        return Vec::new();
    };
    let mut found = Vec::new();
    for (net, pins) in board.nets() {
        let signal = !is_rail(board, net, true) && !is_rail(board, net, false);
        let gpio = pins.iter().any(|p| p.refdes() == mcu && is_gpio(p));
        let clamped = pins.iter().any(|p| p.kind == PartKind::Diode);
        if !signal || !gpio || clamped {
            continue;
        }
        for pin in pins.iter().filter(|p| p.kind == PartKind::Connector) {
            let message = format!(
                "{} brings net \"{}\" off the board to a GPIO with no clamp diode",
                pin.label(),
                pin.net_name
            );
            let help = "add a TVS or ESD diode on the net next to the connector";
            found.push(diagnostic(
                meta,
                Severity::Warning,
                message,
                help,
                vec![pin.id.to_string()],
            ));
        }
    }
    found
}
