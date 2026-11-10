//! Builds the [`BoardBindings`] of one MCU from the netlist and the silicon tables.

use std::collections::BTreeSet;

use crate::netlist::Netlist;
use crate::parser::Circuit;

use super::model::{BoardBindings, BusPin, Ctx};
use super::pins::{McuInfo, PinMapping};

/// Builds the bindings of `mcu` on `circuit`.
pub fn board_bindings(
    board: &str,
    mcu: McuInfo,
    circuit: &Circuit,
    netlist: &Netlist,
) -> BoardBindings {
    let mcu_id = mcu.component_id.clone();
    let ctx = Ctx {
        circuit,
        netlist,
        mcu_id: &mcu_id,
        silicon: crate::silicon::find_target(&mcu.chip),
    };
    let (i2c, spi, uart) = super::buses::buses(&ctx, &mcu.pins);
    let adc = super::adc::inputs(&ctx, &mcu.pins);
    let mut bindings = BoardBindings {
        board: board.to_string(),
        i2c,
        spi,
        uart,
        adc,
        gpio: Vec::new(),
        indicators: Vec::new(),
        parts: super::parts::parts(&ctx),
        mcu,
    };
    bindings.free_pins(&ctx);
    if ctx.silicon.is_some_and(|t| t.family == "Espressif") {
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
    /// Fills `gpio` with the pins no bus or ADC input claims, and `indicators` with the
    /// LEDs and buttons among them.
    fn free_pins(&mut self, ctx: &Ctx) {
        let claimed = self.claimed();
        let free: Vec<&PinMapping> = self
            .mcu
            .pins
            .iter()
            .filter(|p| !claimed.contains(&p.signal_name))
            .collect();
        self.indicators = super::indicators::indicators(ctx, &free);
        self.gpio = free.into_iter().map(BusPin::of).collect();
    }

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
