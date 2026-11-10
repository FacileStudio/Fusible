//! The HAL a generated `board.rs` is written for: crate, config type, and how it names
//! and types a peripheral.

use super::model::BusPin;
use super::view::Family;

/// A HAL with a typed peripheral per pin and bus.
pub struct Hal {
    family: Family,
    /// The crate path, `embassy_stm32`.
    pub krate: &'static str,
    /// The type `init` takes.
    pub config: &'static str,
}

impl Hal {
    /// The HAL of `family`, or `None` when Fusible has no mapping for it.
    pub fn of(family: Family) -> Option<Self> {
        let (krate, config) = match family {
            Family::Stm32 => ("embassy_stm32", "embassy_stm32::Config"),
            Family::Rp => ("embassy_rp", "embassy_rp::config::Config"),
            Family::Esp => ("esp_hal", "esp_hal::Config"),
            Family::Other => return None,
        };
        Some(Self {
            family,
            krate,
            config,
        })
    }

    /// The owned type of peripheral `name`: `Peri<'static, hal::PA5>` for Embassy,
    /// `hal::GPIO48<'static>` for esp-hal.
    pub fn ty(&self, name: &str) -> String {
        match self.family {
            Family::Esp => format!("hal::{name}<'static>"),
            _ => format!("Peri<'static, hal::{name}>"),
        }
    }

    /// The `Peripherals` field of `pin`: `PIN_16` on the RP2040, the pin label elsewhere.
    pub fn pin(&self, pin: &BusPin) -> String {
        match self.family {
            Family::Rp => format!("PIN_{}", pin.number),
            _ => pin.label.clone(),
        }
    }

    /// The `use` line that brings the peripheral types into scope.
    pub fn imports(&self) -> String {
        match self.family {
            Family::Esp => format!("use {}::peripherals as hal;", self.krate),
            _ => format!("use {}::{{peripherals as hal, Peri}};", self.krate),
        }
    }
}
