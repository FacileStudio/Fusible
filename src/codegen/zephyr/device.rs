//! The devicetree facts of one part on a bus: from the manifest, else the part
//! table, else nothing and a reason the user must fill in.

use crate::codegen::model::Part;
use crate::codegen::view::Kind;

use super::table::{self, Address, Entry};
use super::Overrides;

/// What a child node needs.
pub(super) struct Facts {
    pub node: String,
    pub compatible: String,
    pub reg: Option<u16>,
    pub spi_hz: Option<u32>,
}

/// The facts of `name` on a `bus`, or what is missing.
pub(super) fn facts(
    name: &str,
    part: Option<&Part>,
    bus: Kind,
    overrides: &Overrides,
) -> Result<Facts, String> {
    let given = overrides.get(name).cloned().unwrap_or_default();
    let compatible = given.compatible.filter(|c| is_compatible(c));
    let mpn = part.and_then(|p| p.mpn.as_deref());
    let entry = match &compatible {
        Some(_) => None,
        None => mpn.and_then(|m| table::find(m, bus)),
    };
    let compatible = compatible
        .or_else(|| entry.map(|e| e.compatible.to_string()))
        .ok_or_else(|| unknown(bus))?;
    let reg = match bus {
        Kind::I2c => Some(given.reg.map_or_else(|| address(entry, part), Ok)?),
        _ => None,
    };
    let spi_hz = given.spi_max_frequency.or(entry.and_then(|e| e.spi_hz));
    if bus == Kind::Spi && spi_hz.is_none() {
        return Err("set `spi-max-frequency`".to_string());
    }
    Ok(Facts {
        node: entry.map_or_else(|| super::node_name(name), |e| e.node.to_string()),
        compatible,
        reg,
        spi_hz: spi_hz.filter(|_| bus == Kind::Spi),
    })
}

fn unknown(bus: Kind) -> String {
    let needs = match bus {
        Kind::I2c => "`compatible` and `reg`",
        Kind::Spi => "`compatible` and `spi-max-frequency`",
        Kind::Uart => "`compatible`",
    };
    format!("no Zephyr binding is known for this part; set {needs}")
}

fn address(entry: Option<&Entry>, part: Option<&Part>) -> Result<u16, String> {
    let entry = entry.ok_or("set the I2C address `reg`")?;
    match &entry.address {
        Address::Fixed(a) => Ok(*a),
        Address::Strap { pins, low, high } => {
            let tied = pins
                .iter()
                .find_map(|pin| part.and_then(|p| p.tied.get(*pin)));
            match tied {
                Some(crate::codegen::model::Level::Low) => Ok(*low),
                Some(crate::codegen::model::Level::High) => Ok(*high),
                None => Err(format!(
                    "the address is {low:#04x} or {high:#04x} by pin {}, which the netlist does not \
                     tie to ground or a supply; set `reg`",
                    pins.join("/")
                )),
            }
        }
        Address::None => Err("set the I2C address `reg`".to_string()),
    }
}

/// A `vendor,device` binding name with devicetree characters only.
fn is_compatible(text: &str) -> bool {
    !text.is_empty()
        && text
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, ',' | '.' | '_' | '+' | '-'))
}
