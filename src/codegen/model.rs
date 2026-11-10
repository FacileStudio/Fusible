//! One `BoardBindings` model of the MCU wiring that every renderer reads.

use std::collections::BTreeMap;

use crate::netlist::Netlist;
use crate::parser::{Circuit, Component};
use crate::silicon::model::SiliconTarget;
use crate::silicon::PeripheralSignal;

pub use super::indicators::{Indicator, IndicatorKind};
pub use super::parts::{Level, Part};
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

/// A UART TX/RX pair and the devices on its TX line.
#[derive(Debug, Clone, PartialEq)]
pub struct UartPort {
    pub instance: Option<u8>,
    pub tx: BusPin,
    pub rx: BusPin,
    pub devices: Vec<String>,
}

/// Two resistors that divide an input down to an ADC pin, in ohms.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Divider {
    /// From the input to the pin.
    pub top: f64,
    /// From the pin to ground.
    pub bottom: f64,
}

impl Divider {
    /// Input voltage over pin voltage, `(top + bottom) / bottom`.
    pub fn ratio(&self) -> f64 {
        (self.top + self.bottom) / self.bottom
    }
}

/// An analog input and, when two resistors form one, its divider.
#[derive(Debug, Clone, PartialEq)]
pub struct AdcInput {
    pub pin: BusPin,
    /// `(adc, channel)` from the silicon table.
    pub channel: Option<(u8, u8)>,
    pub divider: Option<Divider>,
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
    /// LEDs and buttons among the `gpio` pins whose polarity the netlist shows.
    pub indicators: Vec<Indicator>,
    /// Every part other than the MCU, by designator.
    pub parts: BTreeMap<String, Part>,
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
