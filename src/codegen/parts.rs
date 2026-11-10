//! The parts other than the MCU: part numbers and the pins tied to a fixed level.

use std::collections::BTreeMap;

use crate::linter::roles::{is_ground_name, is_rail_name};
use crate::parser::Port;

use super::model::Ctx;

/// Level of a pin tied straight to a ground or a supply net.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    Low,
    High,
}

/// A part on the board, as a renderer that names devices needs it.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Part {
    /// `manufacturer_part_number` from the Circuit JSON.
    pub mpn: Option<String>,
    /// `ftype` from the Circuit JSON (`simple_chip`, `simple_pin_header`, ...).
    pub kind: Option<String>,
    /// Pins on a ground or supply net, by uppercase port name and hint.
    pub tied: BTreeMap<String, Level>,
}

impl Part {
    /// True for a chip, the only kind of part that is a bus device of its own.
    pub fn is_chip(&self) -> bool {
        self.kind.as_deref().is_some_and(|k| k.contains("chip"))
    }
}

/// Every component but the MCU, by designator.
pub(super) fn parts(ctx: &Ctx) -> BTreeMap<String, Part> {
    ctx.circuit
        .components()
        .filter_map(|c| {
            let id = c.component_id.as_deref().filter(|id| *id != ctx.mcu_id)?;
            let name = c.name.clone().unwrap_or_else(|| id.to_string());
            let part = Part {
                mpn: c.props.manufacturer_part_number.clone(),
                kind: c.kind.clone(),
                tied: tied(ctx, id),
            };
            Some((name, part))
        })
        .collect()
}

fn tied(ctx: &Ctx, id: &str) -> BTreeMap<String, Level> {
    ctx.circuit
        .ports()
        .filter(|p| p.source_component_id.as_deref() == Some(id))
        .filter_map(|p| Some((names(p), level(&net_of(ctx, p.port_id.as_deref()?)?)?)))
        .flat_map(|(names, level)| names.into_iter().map(move |n| (n, level)))
        .collect()
}

/// The uppercase name and hints of a port.
pub(super) fn names(port: &Port) -> Vec<String> {
    port.name
        .iter()
        .chain(port.port_hints.iter().flatten())
        .map(|n| n.to_ascii_uppercase())
        .collect()
}

/// The name of the net a port sits on.
pub(super) fn net_of(ctx: &Ctx, port_id: &str) -> Option<String> {
    let index = *ctx.netlist.port_to_net.get(port_id)?;
    Some(ctx.netlist.nets.get(index)?.display_name().to_string())
}

/// `Low` on a ground net, `High` on a supply net.
pub(super) fn level(net: &str) -> Option<Level> {
    if is_ground_name(net) {
        Some(Level::Low)
    } else if is_rail_name(net) {
        Some(Level::High)
    } else {
        None
    }
}
