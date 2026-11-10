//! Power order: which rail feeds which, and a source-first sort of the rails.
//!
//! A part feeds rail B from rail A when it has an input pin on A and an output pin on B.
//! An output is a `provides_power` pin, a pin named `OUT`, `VOUT`, `VO`, `BAT` or `SYS`,
//! or a diode cathode. An input is a `requires_power` pin, a pin named `IN`, `VIN` or
//! `VI`, or a diode anode. Names and port hints win over attributes, so a `VOUT` marked
//! `requires_power` still reads as an output.

use super::view::{Pin, View};

/// Rail `from` feeds rail `to` through the part `via`.
pub struct Edge {
    pub from: String,
    pub to: String,
    pub via: String,
}

#[derive(PartialEq)]
enum Flow {
    In,
    Out,
}

fn flow(pin: &Pin) -> Option<Flow> {
    let port = pin.port;
    let names: Vec<String> = port
        .name
        .iter()
        .chain(port.port_hints.iter().flatten())
        .map(|n| n.to_ascii_uppercase())
        .collect();
    let has = |list: &[&str]| names.iter().any(|n| list.contains(&n.as_str()));
    let diode = pin.kind().contains("diode");
    if port.attributes.provides_power == Some(true)
        || has(&["OUT", "VOUT", "VO", "BAT", "SYS"])
        || (diode && has(&["CATHODE", "K"]))
    {
        return Some(Flow::Out);
    }
    let input = port.attributes.requires_power == Some(true)
        || has(&["IN", "VIN", "VI"])
        || (diode && has(&["ANODE", "A"]));
    input.then_some(Flow::In)
}

/// Every feed between two of `rails`.
pub fn edges(view: &View, rails: &[String]) -> Vec<Edge> {
    let mut edges = Vec::new();
    for rail in rails {
        for pin in view
            .pins_on(rail)
            .iter()
            .filter(|p| flow(p) == Some(Flow::Out))
        {
            let id = pin.component.component_id.as_deref().unwrap_or_default();
            let inputs = view.pins_of(id).into_iter().filter_map(|port| {
                let net = view.net_of(port)?.display_name().to_string();
                let input = flow(&Pin {
                    component: pin.component,
                    port,
                }) == Some(Flow::In);
                (input && &net != rail && rails.contains(&net)).then_some(net)
            });
            let via = pin.component.name.clone().unwrap_or_else(|| id.to_string());
            edges.extend(inputs.map(|from| Edge {
                from,
                to: rail.clone(),
                via: via.clone(),
            }));
        }
    }
    edges
}

/// Sources first, then each rail after the rails feeding it; ties by voltage, highest
/// (or unknown) first, then by name. Rails caught in a loop come last.
pub fn sort(
    rails: &[String],
    edges: &[Edge],
    voltage: &dyn Fn(&str) -> Option<f64>,
) -> Vec<String> {
    let mut left: Vec<String> = rails.to_vec();
    let mut done: Vec<String> = Vec::new();
    let key = |r: &String| -voltage(r).unwrap_or(f64::INFINITY);
    left.sort_by(|a, b| key(a).total_cmp(&key(b)).then_with(|| a.cmp(b)));
    while !left.is_empty() {
        let ready = left
            .iter()
            .position(|r| {
                edges
                    .iter()
                    .filter(|e| &e.to == r)
                    .all(|e| done.contains(&e.from))
            })
            .unwrap_or(0);
        done.push(left.remove(ready));
    }
    done
}
