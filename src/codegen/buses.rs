//! I2C, SPI and UART buses formed by the MCU pins, matched by role and prefix.

use crate::silicon::PeripheralSignal;

use super::model::{BusPin, Ctx, I2cBus, Signal, SpiBus, UartPort};
use super::pins::PinMapping;
use super::role::{role_of, BusRole};

type Roled<'a> = Vec<(BusRole, String, &'a PinMapping)>;

/// The I2C buses, SPI buses and UART pairs of `pins`.
pub(super) fn buses(ctx: &Ctx, pins: &[PinMapping]) -> (Vec<I2cBus>, Vec<SpiBus>, Vec<UartPort>) {
    let roled: Roled = pins
        .iter()
        .filter_map(|p| role_of(p).map(|(role, prefix)| (role, prefix, p)))
        .collect();
    let i2c = pairs(&roled, BusRole::Scl, BusRole::Sda);
    let uart = pairs(&roled, BusRole::Tx, BusRole::Rx);
    (
        i2c.into_iter().map(|pair| i2c_bus(ctx, pair)).collect(),
        super::spi::spi(ctx, &roled),
        uart.into_iter().map(|pair| uart_port(ctx, pair)).collect(),
    )
}

fn i2c_bus(ctx: &Ctx, (scl, sda): (&PinMapping, &PinMapping)) -> I2cBus {
    let (instance, scl, sda) = two(
        ctx,
        (scl, PeripheralSignal::I2cScl),
        (sda, PeripheralSignal::I2cSda),
    );
    I2cBus {
        instance,
        devices: devices(ctx, &sda.1),
        scl: scl.0,
        sda: sda.0,
    }
}

fn uart_port(ctx: &Ctx, (tx, rx): (&PinMapping, &PinMapping)) -> UartPort {
    let (instance, tx, rx) = two(
        ctx,
        (tx, PeripheralSignal::UartTx),
        (rx, PeripheralSignal::UartRx),
    );
    UartPort {
        instance,
        devices: devices(ctx, &tx.1),
        tx: tx.0,
        rx: rx.0,
    }
}

/// Two lines of one bus with their shared instance; each pin comes back with its net.
fn two(
    ctx: &Ctx,
    a: (&PinMapping, Signal),
    b: (&PinMapping, Signal),
) -> (Option<u8>, (BusPin, String), (BusPin, String)) {
    let mut pins = [(BusPin::of(a.0), a.1), (BusPin::of(b.0), b.1)];
    let instance = assign(ctx, &mut pins);
    let [(pa, _), (pb, _)] = pins;
    (
        instance,
        (pa, a.0.net_name.clone()),
        (pb, b.0.net_name.clone()),
    )
}

/// The first pin with `role` and `prefix`.
pub(super) fn find<'a>(roled: &Roled<'a>, role: BusRole, prefix: &str) -> Option<&'a PinMapping> {
    roled
        .iter()
        .find(|(r, pre, _)| *r == role && pre == prefix)
        .map(|(_, _, p)| *p)
}

/// Each `a` pin with the `b` pin of the same prefix.
fn pairs<'a>(roled: &Roled<'a>, a: BusRole, b: BusRole) -> Vec<(&'a PinMapping, &'a PinMapping)> {
    roled
        .iter()
        .filter(|(role, _, _)| *role == a)
        .filter_map(|(_, prefix, pin)| Some((*pin, find(roled, b, prefix)?)))
        .collect()
}

/// Sets each pin's AF for the shared instance the silicon table finds, and returns it.
pub(super) fn assign(ctx: &Ctx, pins: &mut [(BusPin, Signal)]) -> Option<u8> {
    let found = {
        let want: Vec<(&str, Signal)> = pins.iter().map(|(p, s)| (p.label.as_str(), *s)).collect();
        ctx.instance(&want)
    };
    let (n, afs) = found?;
    for ((pin, _), af) in pins.iter_mut().zip(afs) {
        pin.af = Some(af);
    }
    Some(n)
}

/// Names of the chips and connectors on `net`, passives left out.
pub(super) fn devices(ctx: &Ctx, net: &str) -> Vec<String> {
    const PASSIVE: [&str; 6] = [
        "resistor",
        "capacitor",
        "inductor",
        "diode",
        "led",
        "ferrite",
    ];
    let mut names: Vec<String> = ctx
        .components_on(net)
        .into_iter()
        .filter(|c| {
            !c.kind
                .as_deref()
                .is_some_and(|k| PASSIVE.iter().any(|p| k.contains(p)))
        })
        .filter_map(|c| c.name.clone().or_else(|| c.component_id.clone()))
        .collect();
    names.sort();
    names.dedup();
    names
}
