//! SPI buses: SCK with the MOSI and MISO of its prefix, plus every chip select.

use crate::silicon::PeripheralSignal;

use super::buses::{assign, devices, find};
use super::model::{BusPin, ChipSelect, Ctx, Signal, SpiBus};
use super::pins::PinMapping;
use super::role::BusRole;

type Roled<'a> = Vec<(BusRole, String, &'a PinMapping)>;

/// One bus per SCK pin. A lone bus takes MOSI and MISO of any prefix.
pub(super) fn spi(ctx: &Ctx, roled: &Roled) -> Vec<SpiBus> {
    let sck: Vec<(&String, &PinMapping)> = roled
        .iter()
        .filter(|(r, _, _)| *r == BusRole::Sck)
        .map(|(_, pre, p)| (pre, *p))
        .collect();
    let lone = sck.len() == 1;
    let line = |role: BusRole, prefix: &str| {
        find(roled, role, prefix).or_else(|| lone.then(|| any(roled, role)).flatten())
    };
    let mut buses: Vec<(String, SpiBus)> = sck
        .into_iter()
        .map(|(prefix, pin)| {
            let (mosi, miso) = (line(BusRole::Mosi, prefix), line(BusRole::Miso, prefix));
            (pin.net_name.clone(), bus(ctx, pin, mosi, miso))
        })
        .collect();
    for (_, prefix, pin) in roled.iter().filter(|(r, _, _)| *r == BusRole::Cs) {
        attach(ctx, &mut buses, prefix, pin);
    }
    buses.into_iter().map(|(_, bus)| bus).collect()
}

fn any<'a>(roled: &Roled<'a>, role: BusRole) -> Option<&'a PinMapping> {
    roled
        .iter()
        .find(|(r, _, _)| *r == role)
        .map(|(_, _, p)| *p)
}

fn bus(
    ctx: &Ctx,
    sck: &PinMapping,
    mosi: Option<&PinMapping>,
    miso: Option<&PinMapping>,
) -> SpiBus {
    let mut pins: Vec<(BusPin, Signal)> = vec![(BusPin::of(sck), PeripheralSignal::SpiSck)];
    pins.extend(mosi.map(|p| (BusPin::of(p), PeripheralSignal::SpiMosi as Signal)));
    pins.extend(miso.map(|p| (BusPin::of(p), PeripheralSignal::SpiMiso as Signal)));
    let instance = assign(ctx, &mut pins);
    let mut lines = pins.into_iter().map(|(p, _)| p);
    let sck = lines.next().unwrap_or_else(|| BusPin::of(sck));
    let mosi = mosi.and_then(|_| lines.next());
    let miso = miso.and_then(|_| lines.next());
    SpiBus {
        instance,
        sck,
        mosi,
        miso,
        chip_selects: Vec::new(),
    }
}

/// Puts a chip select on the bus whose SCK net reaches its device, else the bus of its
/// prefix, else the first bus.
fn attach(ctx: &Ctx, buses: &mut [(String, SpiBus)], prefix: &str, pin: &PinMapping) {
    let device = devices(ctx, &pin.net_name).into_iter().next();
    let on_net = |net: &String| {
        device
            .as_ref()
            .is_some_and(|d| devices(ctx, net).contains(d))
    };
    let index = buses
        .iter()
        .position(|(net, _)| on_net(net))
        .or_else(|| {
            buses
                .iter()
                .position(|(net, _)| net.starts_with(prefix) && !prefix.is_empty())
        })
        .or((!buses.is_empty()).then_some(0));
    if let Some(i) = index {
        buses[i].1.chip_selects.push(ChipSelect {
            pin: BusPin::of(pin),
            device,
        });
    }
}
