//! The board as the electrical rules see it: every pin with its part and net.

use std::collections::{BTreeMap, HashMap};

use super::parts::PartKind;
use super::pin::Pin;
use crate::config::NetSpec;
use crate::linter::LintContext;
use crate::parser::{Component, PinAttributes, Port};

fn resolve<'a>(
    ctx: &'a LintContext<'a>,
    parts: &HashMap<&str, &'a Component>,
    port: &'a Port,
) -> Option<Pin<'a>> {
    let id = port.port_id.as_deref()?;
    let prefix = id.split_once('.').map(|(part, _)| part);
    let part_id = port.source_component_id.as_deref().or(prefix).unwrap_or(id);
    let part = parts.get(part_id).copied();
    let net = ctx.netlist.port_to_net.get(id).copied();
    let node = net.and_then(|n| ctx.netlist.nets.get(n));
    Some(Pin {
        port,
        id,
        part,
        part_id,
        kind: PartKind::of(part, part_id),
        net,
        net_name: node.map_or("", |n| n.display_name()),
    })
}

/// Every identified pin of one circuit, indexed by net.
pub struct Board<'a> {
    pub ctx: &'a LintContext<'a>,
    pub pins: Vec<Pin<'a>>,
    by_net: BTreeMap<usize, Vec<usize>>,
}

impl<'a> Board<'a> {
    /// Resolves every `source_port` with an id; ports without one are `malformed_element`'s.
    pub fn new(ctx: &'a LintContext<'a>) -> Self {
        let parts: HashMap<&str, &Component> = ctx
            .circuit
            .components()
            .filter_map(|c| Some((c.component_id.as_deref()?, c)))
            .collect();
        let pins: Vec<Pin<'a>> = ctx
            .circuit
            .ports()
            .filter_map(|port| resolve(ctx, &parts, port))
            .collect();
        let mut by_net: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
        for (index, pin) in pins.iter().enumerate() {
            if let Some(net) = pin.net {
                by_net.entry(net).or_default().push(index);
            }
        }
        Self { ctx, pins, by_net }
    }

    /// Every net that holds at least one pin, with its pins, in netlist order.
    pub fn nets(&self) -> impl Iterator<Item = (usize, Vec<&Pin<'a>>)> + '_ {
        self.by_net
            .iter()
            .map(|(net, pins)| (*net, pins.iter().map(|&i| &self.pins[i]).collect()))
    }

    /// The pins on net `net`.
    pub fn on_net(&self, net: usize) -> impl Iterator<Item = &Pin<'a>> + '_ {
        let pins = self.by_net.get(&net).map_or(&[][..], Vec::as_slice);
        pins.iter().map(|&i| &self.pins[i])
    }

    /// How many endpoints, ports and unlinked pads, the netlist puts on net `net`.
    pub fn endpoints(&self, net: usize) -> usize {
        self.ctx
            .netlist
            .nets
            .get(net)
            .map_or(0, |n| n.endpoints.len())
    }

    /// The manifest's `nets:` entry for net `net`, matched by name.
    pub fn spec(&self, net: usize) -> Option<&'a NetSpec> {
        let name = self.ctx.netlist.nets.get(net)?.display_name();
        self.ctx.design?.nets.get(name)
    }

    /// Whether some pin's attributes satisfy `test`.
    pub fn any(&self, test: impl Fn(&PinAttributes) -> bool) -> bool {
        self.pins.iter().any(|pin| test(pin.attributes()))
    }

    /// Designators of the chips, in file order, each once.
    pub fn chips(&self) -> Vec<&'a str> {
        let mut names: Vec<&str> = Vec::new();
        for pin in self.pins.iter().filter(|p| p.kind == PartKind::Chip) {
            if !names.contains(&pin.refdes()) {
                names.push(pin.refdes());
            }
        }
        names
    }
}
