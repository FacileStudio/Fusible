//! Power rails: which nets are rails, their expected voltage, and where to probe them.
//!
//! A net is a rail when `nets:` gives it a voltage, a pin on it provides power or a
//! voltage, or its name reads as one (`VBUS`, `VDD3V3`, `+5V`). `requires_power` alone
//! does not make a rail: boards set it on signal pins too. The voltage comes from
//! `nets:`, then a pin's `provides_voltage`, then the name (`3V3` is 3.3 V, `VBUS` 5 V).

use crate::config::DesignRules;

use super::order;
use super::view::{is_ground, View};

/// One rail of the bring-up sheet.
pub struct Rail {
    pub name: String,
    pub voltage: Option<f64>,
    /// Test point, else a capacitor pad, else any pin on the rail.
    pub probe: String,
    /// `VBUS via D_USB` for each rail feeding this one.
    pub fed_by: Vec<String>,
}

/// The rails of the board in power order.
pub fn rails(view: &View, design: &DesignRules) -> Vec<Rail> {
    let mut names: Vec<String> = view
        .netlist
        .nets
        .iter()
        .map(|n| n.display_name().to_string())
        .collect();
    names.sort();
    names.dedup();
    names.retain(|n| is_rail(view, design, n));
    let voltage = |name: &str| voltage(view, design, name);
    let edges = order::edges(view, &names);
    order::sort(&names, &edges, &voltage)
        .into_iter()
        .map(|name| Rail {
            voltage: voltage(&name),
            probe: probe(view, &name),
            fed_by: edges
                .iter()
                .filter(|e| e.to == name)
                .map(|e| format!("{} via {}", e.from, e.via))
                .collect(),
            name,
        })
        .collect()
}

fn is_rail(view: &View, design: &DesignRules, net: &str) -> bool {
    let declared = design
        .nets
        .get(net)
        .is_some_and(|spec| spec.voltage.is_some());
    let powered = view.pins_on(net).iter().any(|pin| {
        let a = &pin.port.attributes;
        a.provides_power == Some(true) || a.provides_voltage.is_some()
    });
    !is_ground(net) && (declared || powered || name_voltage(net).is_some() || rail_name(net))
}

/// `VBUS`, `VIN`, `VBAT`, `VSYS`, `VCC`, `VDDA`: a `V` and 2 to 5 letters.
fn rail_name(net: &str) -> bool {
    let rest = net.trim_start_matches('+').to_ascii_uppercase();
    let letters = rest.strip_prefix('V').unwrap_or_default();
    (2..=5).contains(&letters.len()) && letters.chars().all(|c| c.is_ascii_uppercase())
}

fn voltage(view: &View, design: &DesignRules, net: &str) -> Option<f64> {
    let provided = || {
        view.pins_on(net)
            .iter()
            .filter_map(|p| p.port.attributes.provides_voltage)
            .reduce(f64::max)
    };
    design
        .nets
        .get(net)
        .and_then(|s| s.voltage)
        .or_else(provided)
        .or_else(|| name_voltage(net))
}

/// Volts written in a net name: `3V3` 3.3, `5V` 5, `1V8` 1.8, `+12V` 12, `VBUS` 5.
pub fn name_voltage(net: &str) -> Option<f64> {
    let upper = net.to_ascii_uppercase();
    if upper == "VBUS" || upper == "USB_VBUS" {
        return Some(5.0);
    }
    upper.match_indices('V').find_map(|(i, _)| {
        let before: String = upper[..i]
            .chars()
            .rev()
            .take_while(|c| c.is_ascii_digit() || *c == '.')
            .collect();
        let before: String = before.chars().rev().collect();
        let after: String = upper[i + 1..]
            .chars()
            .take_while(char::is_ascii_digit)
            .collect();
        let text = match after.is_empty() || before.contains('.') {
            true => before,
            false => format!("{before}.{after}"),
        };
        text.parse::<f64>().ok().filter(|v| *v > 0.0)
    })
}

fn probe(view: &View, net: &str) -> String {
    let pins = view.pins_on(net);
    let test_point = pins.iter().find(|p| {
        let name = p
            .component
            .name
            .as_deref()
            .unwrap_or_default()
            .to_ascii_uppercase();
        p.kind().contains("test")
            || (name.starts_with("TP") && name[2..].chars().all(|c| c.is_ascii_digit()))
    });
    let capacitor = || pins.iter().find(|p| p.kind().contains("capacitor"));
    match test_point
        .map(|p| (p, true))
        .or_else(|| capacitor().or(pins.first()).map(|p| (p, false)))
    {
        Some((pin, true)) => format!("test point {}", pin.label()),
        Some((pin, false)) => format!("no test point; probe {}", pin.label()),
        None => "no pin on this net".to_string(),
    }
}
