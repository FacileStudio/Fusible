//! Bus role of an MCU pin, read from its net name or the remote port name.
//!
//! The net name speaks for the MCU (`GPS_TX` is the MCU TX). A pin whose net
//! has no meaningful name takes the remote port name, and a remote `TX` is
//! the MCU `RX`.

use super::pins::PinMapping;

/// What one pin does on a bus.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BusRole {
    Scl,
    Sda,
    Sck,
    Mosi,
    Miso,
    Cs,
    Tx,
    Rx,
    Adc,
}

const TOKENS: &[(&[&str], BusRole)] = &[
    (&["SCL", "SIOC"], BusRole::Scl),
    (&["SDA", "SIOD"], BusRole::Sda),
    (&["SCK", "SCLK"], BusRole::Sck),
    (&["MOSI", "COPI"], BusRole::Mosi),
    (&["MISO", "CIPO"], BusRole::Miso),
    (&["CS", "NSS", "SS", "CSN", "NCS"], BusRole::Cs),
    (&["TX", "TXD"], BusRole::Tx),
    (&["RX", "RXD"], BusRole::Rx),
    (&["ADC", "AIN", "SENSE"], BusRole::Adc),
];

/// The role and bus prefix (`GPS` for `GPS_TX`) of an MCU pin, if any.
pub fn role_of(pin: &PinMapping) -> Option<(BusRole, String)> {
    let named = Some(pin.net_name.as_str()).filter(|n| !is_generic(n));
    named.and_then(split).or_else(|| {
        let (role, _) = split(pin.remote_port.as_deref()?)?;
        let prefix = pin.remote_component.clone().unwrap_or_default();
        Some((flip(role), prefix.to_ascii_uppercase()))
    })
}

/// True for net names tscircuit or Fusible made up (`source_net_3`, `U1.1 > .RX`).
fn is_generic(net: &str) -> bool {
    let upper = net.to_ascii_uppercase();
    net.contains('>')
        || ["TRACE_NET", "UNNAMED", "SOURCE_NET", "NET", "UNCONNECTED_"]
            .iter()
            .any(|p| upper.starts_with(p))
}

fn flip(role: BusRole) -> BusRole {
    match role {
        BusRole::Tx => BusRole::Rx,
        BusRole::Rx => BusRole::Tx,
        other => other,
    }
}

/// Finds the last role token of `name`; the tokens before it are the prefix.
fn split(name: &str) -> Option<(BusRole, String)> {
    let upper = name.to_ascii_uppercase();
    let tokens: Vec<&str> = upper
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|t| !t.is_empty())
        .collect();
    let (index, role) = tokens.iter().enumerate().rev().find_map(|(i, t)| {
        let role = TOKENS.iter().find(|(keys, _)| keys.contains(t))?.1;
        Some((i, role))
    })?;
    Some((role, tokens[..index].join("_")))
}
