//! Placement check of the `decoupling_capacitor` rule: a bypass capacitor
//! sits within 3 mm of the IC power pin it serves.

use std::collections::HashMap;

use super::bypass_cap::{chip_power_pin, is_kind, ComponentIndex, CAPACITOR_KINDS};
use super::copper::Shape;
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
        .elements
        .iter()
        .filter_map(|e| match e {
            crate::parser::CircuitElement::PcbPort(p) => {
                Some((p.pcb_port_id.as_deref()?, p.source_port_id.as_deref()?))
            }
            _ => None,
        })
        .collect();
    ctx.circuit
        .pads()
        .filter_map(|pad| {
            let linked = pad.pcb_port_id.as_deref().and_then(|id| pcb_ports.get(id));
            Some((pad.port_id.as_deref().or(linked.copied())?, pad))
        })
        .collect()
}

const SUPPLY_PINS: &[&str] = &[
    "VDD", "VCC", "VIN", "VOUT", "VBAT", "BAT", "VBUS", "VSYS", "VSUP", "AVDD", "DVDD", "3V3", "5V",
];

/// Whether a pin is a supply pin by its own name, so enable, gate and LED pins tied to a rail are not measured.
pub(super) fn is_supply_pin(name: &str) -> bool {
    let upper = name.to_ascii_uppercase();
    SUPPLY_PINS.iter().any(|token| upper.contains(token))
}

fn pin_label(port: &Port) -> String {
    let name = port.name.as_deref().unwrap_or("");
    match port.pin_number {
        Some(n) => format!("pin {n} ({name})"),
        None => format!("pin {name}"),
    }
}

fn nearest_cap<'a>(
    ctx: &'a LintContext<'_>,
    components: &ComponentIndex<'a>,
    pads: &HashMap<&str, &Pad>,
    net: usize,
    pin: &Shape,
) -> Option<(String, f64)> {
    let caps = ctx.circuit.ports().filter_map(|port| {
        let (port_id, comp_id) = (
            port.port_id.as_deref()?,
            port.source_component_id.as_deref()?,
        );
        let on_net = ctx.netlist.port_to_net.get(port_id) == Some(&net);
        let component = components.get(comp_id).copied();
        let is_cap = is_kind(comp_id, component, CAPACITOR_KINDS, 'C');
        let name = component.and_then(|c| c.name.as_deref()).unwrap_or(comp_id);
        (on_net && is_cap).then_some((name, shape(pads.get(port_id)?)?))
    });
    let gaps = caps.map(|(name, cap)| (name.to_string(), pin.gap(&cap)));
    gaps.min_by(|a, b| a.1.total_cmp(&b.1))
}

/// Warnings for power pins whose nearest placed bypass capacitor is farther than 3 mm.
pub(super) fn too_far(ctx: &LintContext<'_>, components: &ComponentIndex<'_>) -> Vec<Diagnostic> {
    let pads = pads_by_port(ctx);
    let mut found = Vec::new();
    for port in ctx.circuit.ports() {
        let pin = chip_power_pin(ctx, components, port).filter(|p| is_supply_pin(p.pin_name));
        let Some(pin) = pin else {
            continue;
        };
        let at = port
            .port_id
            .as_deref()
            .and_then(|id| pads.get(id))
            .and_then(|pad| shape(pad));
        let net = ctx.netlist.by_id.get(&pin.net.id);
        let (Some(at), Some(net)) = (at, net) else {
            continue;
        };
        if let Some((cap, gap)) = nearest_cap(ctx, components, &pads, *net, &at) {
            if gap > MAX_DISTANCE_MM {
                found.push(warning(
                    &cap,
                    pin.comp_name,
                    &pin_label(port),
                    pin.net.display_name(),
                    gap,
                ));
            }
        }
    }
    found
}

fn warning(cap: &str, ic: &str, pin: &str, net: &str, gap: f64) -> Diagnostic {
    Diagnostic {
        rule: "decoupling_capacitor".to_string(),
        message: format!(
            "{cap} decouples {ic} {pin} from {gap:.1} mm away on {net}; place it within 3 mm"
        ),
        help: Some(format!("Shorter loop, lower inductance. Source: {SOURCE}")),
        severity: crate::linter::Severity::Warning,
        standard: Some("layout guideline".to_string()),
        source_url: Some(SOURCE.to_string()),
        elements: vec![cap.to_string(), ic.to_string()],
        ..Default::default()
    }
}
