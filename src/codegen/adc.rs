//! ADC inputs: pins named as analog inputs or fed by a two-resistor divider.

use crate::netlist::NetNode;
use crate::parser::Component;
use crate::silicon::PeripheralSignal;

use super::model::{AdcInput, BusPin, Ctx};
use super::pins::PinMapping;
use super::role::{role_of, BusRole};

/// Every pin with the ADC role or a divider on its net.
pub(super) fn inputs(ctx: &Ctx, pins: &[PinMapping]) -> Vec<AdcInput> {
    pins.iter()
        .filter_map(|pin| {
            let divider = divider(ctx, &pin.net_name);
            let named = role_of(pin).is_some_and(|(role, _)| role == BusRole::Adc);
            (named || divider.is_some()).then(|| AdcInput {
                pin: BusPin::of(pin),
                channel: channel(ctx, &pin.pin_label),
                divider,
            })
        })
        .collect()
}

fn channel(ctx: &Ctx, label: &str) -> Option<(u8, u8)> {
    let spec = ctx.silicon?.find_pin(label)?;
    spec.functions.iter().find_map(|f| match f.signal {
        PeripheralSignal::AdcChannel(adc, ch) => Some((adc, ch)),
        PeripheralSignal::Adc(ch) => Some((0, ch)),
        _ => None,
    })
}

/// `(top + bottom) / bottom` when exactly two resistors touch `net` and one goes to ground.
fn divider(ctx: &Ctx, net: &str) -> Option<f64> {
    let resistors: Vec<&Component> = ctx
        .components_on(net)
        .into_iter()
        .filter(|c| c.kind.as_deref().is_some_and(|k| k.contains("resistor")))
        .collect();
    let [a, b] = resistors.as_slice() else {
        return None;
    };
    let (bottom, top) = match (to_ground(ctx, a, net), to_ground(ctx, b, net)) {
        (true, false) => (a, b),
        (false, true) => (b, a),
        _ => return None,
    };
    let (top, bottom) = (top.props.resistance?, bottom.props.resistance?);
    (bottom > 0.0).then(|| (top + bottom) / bottom)
}

/// True when the other end of `resistor` sits on a ground net.
fn to_ground(ctx: &Ctx, resistor: &Component, net: &str) -> bool {
    let id = resistor.component_id.as_deref();
    let touches = |n: &&NetNode| n.endpoints.iter().any(|e| e.component_id.as_deref() == id);
    ctx.netlist
        .nets
        .iter()
        .filter(touches)
        .filter(|n| n.display_name() != net)
        .any(|n| is_ground(n.display_name()))
}

pub(super) fn is_ground(net: &str) -> bool {
    let upper = net.to_ascii_uppercase();
    upper.contains("GND") || upper == "VSS" || upper == "0V"
}
