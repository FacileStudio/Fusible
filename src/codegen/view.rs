//! A uniform view of the I2C, SPI and UART buses for the renderers.

use super::model::{BoardBindings, BusPin, SpiBus};

/// Silicon family, which picks the peripheral and pin names a HAL uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Family {
    Stm32,
    Rp,
    Esp,
    Other,
}

impl Family {
    pub fn of(chip: &str) -> Self {
        match crate::silicon::find_target(chip).map(|t| t.family) {
            Some("STM32") => Self::Stm32,
            Some("Raspberry Pi") => Self::Rp,
            Some("Espressif") => Self::Esp,
            _ => Self::Other,
        }
    }
}

/// Kind of a bus.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    I2c,
    Spi,
    Uart,
}

/// One bus: its peripheral name (`I2C1`, `USART2`, `SPI_BUS0` without a known
/// instance), its lines by role, and what hangs on it.
pub struct Bus<'a> {
    pub kind: Kind,
    pub name: String,
    pub instance: Option<u8>,
    pub lines: Vec<(&'static str, &'a BusPin)>,
    pub devices: Vec<String>,
    pub chip_selects: Vec<(&'a BusPin, Option<&'a str>)>,
}

/// Every bus of `b` in a fixed order: I2C, SPI, then UART.
pub fn buses(b: &BoardBindings) -> Vec<Bus<'_>> {
    let family = Family::of(&b.mcu.chip);
    let i2c = b.i2c.iter().enumerate().map(|(i, bus)| Bus {
        kind: Kind::I2c,
        name: name("I2C", bus.instance, i),
        instance: bus.instance,
        lines: vec![("scl", &bus.scl), ("sda", &bus.sda)],
        devices: bus.devices.clone(),
        chip_selects: Vec::new(),
    });
    let spi = b.spi.iter().enumerate().map(|(i, bus)| spi(bus, i));
    let uart = b.uart.iter().enumerate().map(|(i, port)| Bus {
        kind: Kind::Uart,
        name: name(
            uart_prefix(family, &b.mcu.chip, port.instance),
            port.instance,
            i,
        ),
        instance: port.instance,
        lines: vec![("tx", &port.tx), ("rx", &port.rx)],
        devices: Vec::new(),
        chip_selects: Vec::new(),
    });
    i2c.chain(spi).chain(uart).collect()
}

fn spi(bus: &SpiBus, index: usize) -> Bus<'_> {
    let lines = [
        ("sck", Some(&bus.sck)),
        ("mosi", bus.mosi.as_ref()),
        ("miso", bus.miso.as_ref()),
    ];
    let cs = &bus.chip_selects;
    Bus {
        kind: Kind::Spi,
        name: name("SPI", bus.instance, index),
        instance: bus.instance,
        lines: lines
            .into_iter()
            .filter_map(|(role, pin)| Some((role, pin?)))
            .collect(),
        devices: cs.iter().filter_map(|c| c.device.clone()).collect(),
        chip_selects: cs.iter().map(|c| (&c.pin, c.device.as_deref())).collect(),
    }
}

fn name(prefix: &str, instance: Option<u8>, index: usize) -> String {
    match instance {
        Some(n) => format!("{prefix}{n}"),
        None => format!("{prefix}_BUS{index}"),
    }
}

/// STM32 names USART1-3, 6 and 10 but UART4, 5, 7 and 8; G0 parts call them all USART.
fn uart_prefix(family: Family, chip: &str, instance: Option<u8>) -> &'static str {
    let g0 = chip
        .to_ascii_lowercase()
        .replace(['-', '_', ' '], "")
        .starts_with("stm32g0");
    match (family, instance) {
        (Family::Stm32, Some(4 | 5 | 7 | 8)) if !g0 => "UART",
        (Family::Stm32, _) => "USART",
        _ => "UART",
    }
}
