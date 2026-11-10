//! Placement check of the `decoupling_capacitor` rule: a bypass capacitor
//! sits within 3 mm of the IC power pin it serves.

use std::collections::HashMap;

use super::copper::Shape;
use super::erc::parts::PartKind;
use super::erc::{Board, Pin};
use super::pad_geometry::pad_box;
use crate::linter::{Diagnostic, LintContext};
use crate::parser::{Pad, Port};

const MAX_DISTANCE_MM: f64 = 3.0;
const SOURCE: &str =
    "https://www.protoexpress.com/blog/decoupling-capacitor-placement-guidelines-pcb-design/";

fn shape(pad: &Pad) -> Option<Shape> {
    let (x, y, hw, hh) = pad_box(pad)?;
    Some(Shape::Rect { c: (x, y), hw, hh })
}

fn pads_by_port<'a>(ctx: &'a LintContext<'_>) -> HashMap<&'a str, &'a Pad> {
    let pcb_ports: HashMap<&str, &str> = ctx
        .circuit
        .pcb_ports()
        .filter_map(|p| Some((p.pcb_port_id.as_deref()?, p.source_port_id.as_deref()?)))
        .collect();
    ctx.circuit
        .pads()
        .filter_map(|pad| {
            let linked = pad.pcb_port_id.as_deref().and_then(|id| pcb_ports.get(id));
            Some((pad.port_id.as_deref().or(linked.copied())?, pad))
        })
        .collect()
}

fn pin_label(port: &Port) -> String {
    let name = port.name.as_deref().unwrap_or("");
    match port.pin_number {
        Some(n) => format!("pin {n} ({name})"),
        None => format!("pin {name}"),
    }
}

fn nearest_cap<'a>(
    board: &Board<'a>,
    pads: &HashMap<&str, &Pad>,
    net: usize,
    at: &Shape,
) -> Option<(&'a str, &'a str, f64)> {
    let caps = board.on_net(net).filter(|p| p.kind == PartKind::Capacitor);
    let gaps = caps.filter_map(|cap| {
        let pad = shape(pads.get(cap.id)?)?;
        Some((cap.refdes(), cap.part_id, at.gap(&pad)))
    });
    gaps.min_by(|a, b| a.2.total_cmp(&b.2))
}

/// Warnings for power pins whose nearest placed bypass capacitor is farther than 3 mm.
pub(super) fn too_far(board: &Board<'_>, pins: &[&Pin<'_>]) -> Vec<Diagnostic> {
    let pads = pads_by_port(board.ctx);
    let mut found = Vec::new();
    for pin in pins {
        let at = pads.get(pin.id).and_then(|pad| shape(pad));
        let (Some(at), Some(net)) = (at, pin.net) else {
            continue;
        };
        if let Some((cap, cap_id, gap)) = nearest_cap(board, &pads, net, &at) {
            if gap > MAX_DISTANCE_MM {
                found.push(warning(pin, cap, cap_id, gap));
            }
        }
    }
    found
}

fn warning(pin: &Pin<'_>, cap: &str, cap_id: &str, gap: f64) -> Diagnostic {
    let (ic, label, net) = (pin.refdes(), pin_label(pin.port), pin.net_name);
    Diagnostic {
        rule: "decoupling_capacitor".to_string(),
        message: format!(
            "{cap} decouples {ic} {label} from {gap:.1} mm away on {net}; place it within 3 mm"
        ),
        help: Some(format!("Shorter loop, lower inductance. Source: {SOURCE}")),
        severity: crate::linter::Severity::Warning,
        standard: Some("layout guideline".to_string()),
        source_url: Some(SOURCE.to_string()),
        elements: vec![pin.id.to_string(), cap_id.to_string()],
        ..Default::default()
    }
}
