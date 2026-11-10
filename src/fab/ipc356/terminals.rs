//! The test points of a board: SMT pads, plated and non-plated holes, vias.

use std::collections::HashMap;

use super::Terminal;
use crate::netlist::Netlist;
use crate::parser::{Circuit, Pad};

/// Port and part names, to label each terminal `R1-2`.
struct Names<'a> {
    pcb_port: HashMap<&'a str, &'a str>,
    port: HashMap<&'a str, (&'a str, String)>,
    part: HashMap<&'a str, &'a str>,
}

impl<'a> Names<'a> {
    fn new(circuit: &'a Circuit) -> Self {
        let pcb_port = circuit
            .pcb_ports()
            .filter_map(|p| Some((p.pcb_port_id.as_deref()?, p.source_port_id.as_deref()?)));
        let port = circuit.ports().filter_map(|p| {
            let pin = p
                .pin_number
                .map(|n| n.to_string())
                .or_else(|| p.name.clone());
            Some((
                p.port_id.as_deref()?,
                (p.source_component_id.as_deref()?, pin?),
            ))
        });
        let part = circuit.components().filter_map(|c| {
            let id = c.component_id.as_deref()?;
            Some((id, c.name.as_deref().unwrap_or(id)))
        });
        Self {
            pcb_port: pcb_port.collect(),
            port: port.collect(),
            part: part.collect(),
        }
    }

    /// The source port and its `(designator, pin)` behind a PCB port.
    fn label(&self, source_port: Option<&str>, pcb_port: Option<&str>) -> Option<(String, String)> {
        let port = source_port.or_else(|| self.pcb_port.get(pcb_port?).copied())?;
        let (part, pin) = self.port.get(port)?;
        Some((self.part.get(part).unwrap_or(part).to_string(), pin.clone()))
    }
}

pub(super) fn net_of(netlist: &Netlist, index: Option<&usize>) -> String {
    let node = index.and_then(|&i| netlist.nets.get(i));
    node.map_or_else(|| "N/C".to_string(), |n| n.display_name().to_string())
}

fn pad_size(pad: &Pad) -> Option<((f64, f64), (f64, f64))> {
    if let (Some(x), Some(y)) = (pad.x, pad.y) {
        let size = match (pad.width, pad.height, pad.radius) {
            (Some(w), Some(h), _) => (w, h),
            (_, _, Some(r)) => (2.0 * r, 2.0 * r),
            _ => return None,
        };
        return Some(((x, y), size));
    }
    let points = pad.points.as_deref().filter(|p| !p.is_empty())?;
    let (lo, hi) = points.iter().fold(
        ((f64::MAX, f64::MAX), (f64::MIN, f64::MIN)),
        |(lo, hi), p| {
            (
                (lo.0.min(p.x), lo.1.min(p.y)),
                (hi.0.max(p.x), hi.1.max(p.y)),
            )
        },
    );
    Some((
        ((lo.0 + hi.0) / 2.0, (lo.1 + hi.1) / 2.0),
        (hi.0 - lo.0, hi.1 - lo.1),
    ))
}

fn smt(circuit: &Circuit, netlist: &Netlist, names: &Names<'_>, layers: u8) -> Vec<Terminal> {
    let pads = circuit.pads().filter_map(|pad| {
        let (at, size) = pad_size(pad)?;
        let net = net_of(
            netlist,
            pad.pad_id
                .as_deref()
                .and_then(|id| netlist.pad_to_net.get(id)),
        );
        let (refdes, pin) = names
            .label(pad.port_id.as_deref(), pad.pcb_port_id.as_deref())
            .unwrap_or_default();
        let bottom = pad
            .layer
            .as_deref()
            .is_some_and(|l| l.eq_ignore_ascii_case("bottom"));
        Some(Terminal {
            net,
            refdes,
            pin,
            drill: None,
            access: if bottom { layers } else { 1 },
            at,
            size,
        })
    });
    pads.collect()
}

fn plated(circuit: &Circuit, netlist: &Netlist, names: &Names<'_>) -> Vec<Terminal> {
    let holes = circuit.model().plated_holes.iter().filter_map(|h| {
        let drill = h
            .hole_diameter
            .or_else(|| Some(h.hole_width?.min(h.hole_height?)))?;
        let size = h.outer_diameter.map(|d| (d, d));
        let size = size
            .or_else(|| h.outer_width.zip(h.outer_height))
            .or_else(|| h.rect_pad_width.zip(h.rect_pad_height));
        let label = names.label(None, h.pcb_port_id.as_deref());
        let port = h.pcb_port_id.as_deref().and_then(|p| names.pcb_port.get(p));
        let net = net_of(netlist, port.and_then(|p| netlist.port_to_net.get(*p)));
        let (refdes, pin) = label.unwrap_or_default();
        Some(Terminal {
            net,
            refdes,
            pin,
            drill: Some((drill, true)),
            access: 0,
            at: (h.x, h.y),
            size: size.unwrap_or((drill, drill)),
        })
    });
    holes.collect()
}

/// Every terminal of `circuit`, SMT pads first, then plated holes, vias and
/// non-plated holes. `layers` is the copper layer count, the access code of
/// the bottom side.
pub(super) fn terminals(circuit: &Circuit, netlist: &Netlist, layers: u8) -> Vec<Terminal> {
    let names = Names::new(circuit);
    let mut all = smt(circuit, netlist, &names, layers);
    all.extend(plated(circuit, netlist, &names));
    all.extend(super::holes::drilled(circuit, netlist));
    all
}
