//! Strap pins: the MCU pins whose level at reset picks a boot mode, and how the board
//! sets them.

use crate::codegen::McuInfo;
use crate::silicon::{SiliconTarget, StrappingRequirement};

use super::view::{is_ground, View};

/// One strap pin of the bring-up sheet.
pub struct Strap {
    pub pin: String,
    pub net: String,
    /// `high` or `low` at reset.
    pub level: &'static str,
    pub purpose: &'static str,
    /// What the board does to the pin: `pulled up by R_PU to VDD3V3`, `floating`, ...
    pub state: String,
}

/// The strap pins of `mcu` that the silicon table lists.
pub fn straps(view: &View, mcu: &McuInfo, silicon: &SiliconTarget) -> Vec<Strap> {
    silicon
        .pins
        .iter()
        .filter_map(|spec| {
            let (level, purpose) = match spec.strapping? {
                StrappingRequirement::MustBeHigh { purpose, .. } => ("high", purpose),
                StrappingRequirement::MustBeLow { purpose, .. } => ("low", purpose),
            };
            let net = strap_net(view, mcu, silicon, spec.name);
            let state = net
                .as_deref()
                .map_or_else(|| "not connected".to_string(), |n| state(view, n));
            Some(Strap {
                pin: spec.name.to_string(),
                net: net.unwrap_or_default(),
                level,
                purpose,
                state,
            })
        })
        .collect()
}

/// The net of the MCU port that the silicon table names `pin`.
fn strap_net(view: &View, mcu: &McuInfo, silicon: &SiliconTarget, pin: &str) -> Option<String> {
    let named = |name: &str| silicon.find_pin(name).is_some_and(|s| s.name == pin);
    let port = view
        .pins_of(&mcu.component_id)
        .into_iter()
        .find(|p| p.name.as_deref().is_some_and(named))?;
    Some(view.net_of(port)?.display_name().to_string())
}

/// How the net is driven at reset: tied, pulled through a resistor, or floating.
fn state(view: &View, net: &str) -> String {
    if is_ground(net) {
        return format!("tied to {net}");
    }
    let pulls: Vec<String> = view
        .pins_on(net)
        .iter()
        .filter(|p| p.kind().contains("resistor"))
        .filter_map(|r| {
            let id = r.component.component_id.as_deref()?;
            let other = view
                .pins_of(id)
                .into_iter()
                .find(|p| p.port_id != r.port.port_id)?;
            let far = view.net_of(other)?.display_name();
            let name = r.component.name.as_deref().unwrap_or(id);
            let dir = if is_ground(far) { "down" } else { "up" };
            Some(format!("pulled {dir} by {name} to {far}"))
        })
        .collect();
    match pulls.is_empty() {
        true => "no pull resistor (internal default)".to_string(),
        false => pulls.join(", "),
    }
}
