//! Debug and programming connectors: connectors with a pin on an MCU debug, boot,
//! reset, UART or USB net, and their full pinout.

use super::view::{Pin, View};

/// One connector pin: its name, its net, and the MCU pin on that net.
pub struct ConnectorPin {
    pub pin: String,
    pub net: String,
    pub mcu: Option<String>,
}

/// A connector that reaches the MCU's debug or programming pins.
pub struct Connector {
    pub name: String,
    pub pins: Vec<ConnectorPin>,
}

const DEBUG_WORDS: [&str; 16] = [
    "SWD", "SWCLK", "SWDIO", "JTAG", "TMS", "TCK", "TDI", "TDO", "RST", "RESET", "BOOT", "TX",
    "RX", "USB", "D+", "D-",
];

fn is_connector(pin: &Pin) -> bool {
    let name = pin.component.name.as_deref().unwrap_or_default();
    pin.kind().contains("connector") || pin.kind().contains("header") || name.starts_with('J')
}

fn is_debug(net: &str, mcu_pin: &str) -> bool {
    let text = format!("{net} {mcu_pin}").to_ascii_uppercase();
    DEBUG_WORDS.iter().any(|w| text.contains(w)) || text.contains(" EN") || text.contains("PU")
}

/// The connectors with at least one pin on an MCU debug or programming net.
pub fn connectors(view: &View) -> Vec<Connector> {
    let mut ids: Vec<String> = view
        .circuit
        .ports()
        .filter_map(|port| {
            let component = view.component(port.source_component_id.as_deref()?)?;
            is_connector(&Pin { component, port }).then(|| component.component_id.clone())?
        })
        .collect();
    ids.sort();
    ids.dedup();
    ids.iter().filter_map(|id| connector(view, id)).collect()
}

fn connector(view: &View, id: &str) -> Option<Connector> {
    let pins: Vec<ConnectorPin> = view
        .pins_of(id)
        .into_iter()
        .map(|port| {
            let net = view
                .net_of(port)
                .map(|n| n.display_name().to_string())
                .unwrap_or_default();
            let mcu = view.pins_on(&net).into_iter().find(|p| view.is_mcu(p));
            ConnectorPin {
                pin: port.name.clone().unwrap_or_default(),
                mcu: mcu.map(|p| p.port.name.clone().unwrap_or_default()),
                net,
            }
        })
        .collect();
    let debug = pins
        .iter()
        .any(|p| p.mcu.as_deref().is_some_and(|m| is_debug(&p.net, m)));
    let name = view
        .component(id)?
        .name
        .clone()
        .unwrap_or_else(|| id.to_string());
    debug.then_some(Connector { name, pins })
}
