//! One `BoardBindings` model of the MCU wiring that every renderer reads.

use std::collections::BTreeSet;

use crate::netlist::Netlist;
use crate::parser::{Circuit, Component};
use crate::silicon::model::SiliconTarget;
use crate::silicon::PeripheralSignal;

use super::pins::{McuInfo, PinMapping};

/// One MCU pin as a bus or ADC uses it.
#[derive(Debug, Clone, PartialEq)]
pub struct BusPin {
    /// Identifier of the pin in `pins.rs` (`GPS_TX`).
    pub signal: String,
    /// Silicon pin name (`PA9`, `GPIO4`).
    pub label: String,
    pub number: u32,
    /// Alternate function number, when the silicon table knows it.
    pub af: Option<u8>,
}

/// An I2C bus and the devices hanging on its SDA line.
#[derive(Debug, Clone, PartialEq)]
pub struct I2cBus {
    pub instance: Option<u8>,
    pub scl: BusPin,
    pub sda: BusPin,
    pub devices: Vec<String>,
}

/// A chip-select line and the device it selects.
#[derive(Debug, Clone, PartialEq)]
pub struct ChipSelect {
    pub pin: BusPin,
    pub device: Option<String>,
}

/// An SPI bus with its chip selects.
#[derive(Debug, Clone, PartialEq)]
pub struct SpiBus {
    pub instance: Option<u8>,
    pub sck: BusPin,
    pub mosi: Option<BusPin>,
    pub miso: Option<BusPin>,
    pub chip_selects: Vec<ChipSelect>,
}

/// A UART TX/RX pair.
#[derive(Debug, Clone, PartialEq)]
pub struct UartPort {
    pub instance: Option<u8>,
    pub tx: BusPin,
    pub rx: BusPin,
}

/// An analog input and, when two resistors form one, its divider ratio.
#[derive(Debug, Clone, PartialEq)]
pub struct AdcInput {
    pub pin: BusPin,
    /// `(adc, channel)` from the silicon table.
    pub channel: Option<(u8, u8)>,
    /// Input voltage over pin voltage, `(top + bottom) / bottom`.
    pub divider: Option<f64>,
}

/// Everything the renderers emit for one board.
#[derive(Debug, Clone, PartialEq)]
pub struct BoardBindings {
    pub board: String,
    pub mcu: McuInfo,
    pub i2c: Vec<I2cBus>,
    pub spi: Vec<SpiBus>,
    pub uart: Vec<UartPort>,
    pub adc: Vec<AdcInput>,
    /// Pins no bus or ADC input claims.
    pub gpio: Vec<BusPin>,
}

/// A peripheral signal of instance `n`, such as `PeripheralSignal::I2cScl`.
pub(super) type Signal = fn(u8) -> PeripheralSignal;

/// What the bus builders read: the circuit, its netlist and the silicon table.
pub(super) struct Ctx<'a> {
    pub circuit: &'a Circuit,
    pub netlist: &'a Netlist,
    pub mcu_id: &'a str,
    pub silicon: Option<&'static SiliconTarget>,
}

impl Ctx<'_> {
    /// Components other than the MCU on `net`, by id.
    pub fn components_on(&self, net: &str) -> Vec<&Component> {
        let ids: Vec<&str> = self
            .netlist
            .nets
            .iter()
            .filter(|n| n.display_name() == net)
            .flat_map(|n| n.endpoints.iter().filter_map(|e| e.component_id.as_deref()))
            .filter(|id| *id != self.mcu_id)
            .collect();
        self.circuit
            .components()
            .filter(|c| {
                c.component_id
                    .as_deref()
                    .is_some_and(|id| ids.contains(&id))
            })
            .collect()
    }

    /// The smallest instance `n` for which every pin offers `signal(n)`, with each AF.
    pub fn instance(&self, want: &[(&str, Signal)]) -> Option<(u8, Vec<u8>)> {
        let target = self.silicon?;
        (0..=9).find_map(|n| {
            let afs = want
                .iter()
                .map(|(label, signal)| Some(target.find_pin(label)?.get_af(&signal(n))?.af_number))
                .collect::<Option<Vec<u8>>>()?;
            Some((n, afs))
        })
    }
}

impl BusPin {
    pub fn of(pin: &PinMapping) -> Self {
        Self {
            signal: pin.signal_name.clone(),
            label: pin.pin_label.clone(),
            number: pin.pin_number,
            af: None,
        }
    }
}

/// Builds the bindings of `mcu` on `circuit`.
pub fn board_bindings(
    board: &str,
    mcu: McuInfo,
    circuit: &Circuit,
    netlist: &Netlist,
) -> BoardBindings {
    let ctx = Ctx {
        circuit,
        netlist,
        mcu_id: &mcu.component_id,
        silicon: crate::silicon::find_target(&mcu.chip),
    };
    let (i2c, spi, uart) = super::buses::buses(&ctx, &mcu.pins);
    let adc = super::adc::inputs(&ctx, &mcu.pins);
    let matrix = ctx.silicon.is_some_and(|t| t.family == "Espressif");
    let mut bindings = BoardBindings {
        board: board.to_string(),
        i2c,
        spi,
        uart,
        adc,
        gpio: Vec::new(),
        mcu,
    };
    let claimed = bindings.claimed();
    bindings.gpio = bindings
        .mcu
        .pins
        .iter()
        .filter(|p| !claimed.contains(&p.signal_name))
        .map(BusPin::of)
        .collect();
    if matrix {
        bindings.fill_matrix();
    }
    bindings
}

/// Gives each empty slot the lowest instance from `base` that no other slot uses.
fn fill(mut slots: Vec<&mut Option<u8>>, base: u8) {
    let mut used: Vec<u8> = slots.iter().filter_map(|s| **s).collect();
    for slot in slots.iter_mut().filter(|s| s.is_none()) {
        let n = (base..=u8::MAX).find(|n| !used.contains(n)).unwrap_or(base);
        used.push(n);
        **slot = Some(n);
    }
}

impl BoardBindings {
    /// The ESP32 GPIO matrix routes any bus to any pin: a bus the IO MUX did not place
    /// takes the lowest free instance (I2C from 0, SPI from 2 as SPI0/1 drive the flash,
    /// UART from 0).
    fn fill_matrix(&mut self) {
        fill(self.i2c.iter_mut().map(|b| &mut b.instance).collect(), 0);
        fill(self.spi.iter_mut().map(|b| &mut b.instance).collect(), 2);
        fill(self.uart.iter_mut().map(|u| &mut u.instance).collect(), 0);
    }

    /// Signal names of every pin a bus or ADC input claims.
    fn claimed(&self) -> BTreeSet<String> {
        let i2c = self.i2c.iter().flat_map(|b| [&b.scl, &b.sda]);
        let uart = self.uart.iter().flat_map(|u| [&u.tx, &u.rx]);
        let spi = self.spi.iter().flat_map(|b| {
            let cs = b.chip_selects.iter().map(|c| &c.pin);
            std::iter::once(&b.sck)
                .chain(&b.mosi)
                .chain(&b.miso)
                .chain(cs)
        });
        i2c.chain(uart)
            .chain(spi)
            .chain(self.adc.iter().map(|a| &a.pin))
            .map(|p| p.signal.clone())
            .collect()
    }
}
