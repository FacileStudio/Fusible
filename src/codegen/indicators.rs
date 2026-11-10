//! LEDs and buttons on free GPIOs, with the polarity the netlist proves.
//!
//! An LED counts when its anode or cathode sits on the pin's net, directly or
//! through one series resistor: anode on the pin is active high, cathode is
//! active low. A button counts when one side sits on the pin's net and every
//! other side on ground (active low) or on a supply (active high). Anything
//! else stays a plain GPIO.

use crate::parser::{Component, Port};

use super::model::{BusPin, Ctx};
use super::parts::{level, names, net_of, Level};
use super::pins::PinMapping;

/// What a free GPIO drives or reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndicatorKind {
    Led,
    Button,
}

/// An LED or a button on one MCU pin.
#[derive(Debug, Clone, PartialEq)]
pub struct Indicator {
    pub kind: IndicatorKind,
    pub pin: BusPin,
    /// Designator of the LED or button.
    pub device: String,
    pub active_low: bool,
}

/// The LEDs and buttons of `pins`; a part two pins reach counts for neither.
pub(super) fn indicators(ctx: &Ctx, pins: &[&PinMapping]) -> Vec<Indicator> {
    let found: Vec<Indicator> = pins
        .iter()
        .filter_map(|pin| {
            let (kind, part, active_low) = led(ctx, &pin.net_name)
                .map(|(c, low)| (IndicatorKind::Led, c, low))
                .or_else(|| {
                    button(ctx, &pin.net_name).map(|(c, l)| (IndicatorKind::Button, c, l))
                })?;
            Some(Indicator {
                kind,
                pin: BusPin::of(pin),
                device: part.name.clone().or_else(|| part.component_id.clone())?,
                active_low,
            })
        })
        .collect();
    let once = |i: &Indicator| found.iter().filter(|o| o.device == i.device).count() == 1;
    found.iter().filter(|i| once(i)).cloned().collect()
}

/// An LED on `net`, or behind one series resistor whose far end is a signal net.
fn led<'a>(ctx: &'a Ctx, net: &str) -> Option<(&'a Component, bool)> {
    let direct = ends(ctx, net);
    let through: Vec<(&Component, &Port)> = direct
        .iter()
        .filter(|(c, _)| is(c, &["resistor"]))
        .flat_map(|(c, p)| others(ctx, c, p))
        .filter(|other| level(other).is_none())
        .flat_map(|other| ends(ctx, &other))
        .collect();
    direct.into_iter().chain(through).find_map(|(c, p)| {
        let port = names(p);
        let has = |keys: &[&str]| port.iter().any(|n| keys.contains(&n.as_str()));
        let low = match (has(&["ANODE", "POS"]), has(&["CATHODE", "NEG"])) {
            (true, false) => false,
            (false, true) => true,
            _ => return None,
        };
        is(c, &["led"]).then_some((c, low))
    })
}

fn button<'a>(ctx: &'a Ctx, net: &str) -> Option<(&'a Component, bool)> {
    let (part, port) = ends(ctx, net)
        .into_iter()
        .find(|(c, _)| is(c, &["button", "switch"]))?;
    let levels: Vec<Option<Level>> = others(ctx, part, port)
        .iter()
        .filter(|other| *other != net)
        .map(|other| level(other))
        .collect();
    let all = |want: Level| !levels.is_empty() && levels.iter().all(|l| *l == Some(want));
    match (all(Level::Low), all(Level::High)) {
        (true, _) => Some((part, true)),
        (_, true) => Some((part, false)),
        _ => None,
    }
}

/// Every port on `net` of a component other than the MCU, with its component.
fn ends<'a>(ctx: &'a Ctx, net: &str) -> Vec<(&'a Component, &'a Port)> {
    ctx.circuit
        .ports()
        .filter(|p| p.source_component_id.as_deref() != Some(ctx.mcu_id))
        .filter(|p| {
            p.port_id
                .as_deref()
                .and_then(|id| net_of(ctx, id))
                .as_deref()
                == Some(net)
        })
        .filter_map(|p| {
            let id = p.source_component_id.as_deref()?;
            let part = ctx
                .circuit
                .components()
                .find(|c| c.component_id.as_deref() == Some(id))?;
            Some((part, p))
        })
        .collect()
}

/// The nets of the other ports of `part`.
fn others(ctx: &Ctx, part: &Component, port: &Port) -> Vec<String> {
    ctx.circuit
        .ports()
        .filter(|p| p.source_component_id == part.component_id && p.port_id != port.port_id)
        .filter_map(|p| net_of(ctx, p.port_id.as_deref()?))
        .collect()
}

fn is(part: &Component, kinds: &[&str]) -> bool {
    part.kind
        .as_deref()
        .is_some_and(|k| kinds.iter().any(|want| k.contains(want)))
}
