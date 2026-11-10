//! What differs per SoC in a Zephyr devicetree: GPIO and ADC node labels, the
//! pinctrl header, and ADC resolution.
//!
//! Labels come from the Zephyr SoC files: `gpioa`.. and `adc1`.. on STM32
//! (`dts/arm/st/`), `gpio0`, `gpio0_hi` (GPIO32-47 of the RP2350B) and `adc`
//! on RP2040/RP2350 (`dts/arm/raspberrypi/rpi_pico/rp2040.dtsi`,
//! `dts/vendor/raspberrypi/rpi_pico/rp2350.dtsi`), `gpio0`, `gpio1` on ESP32.

use crate::codegen::model::BusPin;
use crate::codegen::view::Family;
use crate::silicon::model::parse_port_pin;

/// The SoC conventions of one board.
pub(super) struct Soc {
    pub family: Family,
    target: &'static str,
    h7: bool,
}

impl Soc {
    pub fn of(chip: &str) -> Self {
        let clean = chip.to_ascii_lowercase().replace(['-', '_', ' '], "");
        Self {
            family: Family::of(chip),
            target: crate::silicon::find_target(chip).map_or("", |t| t.name),
            h7: clean.starts_with("stm32h7"),
        }
    }

    /// True when the silicon table resolved the chip to `target` (`rp2350b`).
    pub fn is(&self, target: &str) -> bool {
        self.target == target
    }

    /// The `zephyr/dt-bindings/pinctrl/` header defining the pinmux macros, for
    /// families with pinctrl groups.
    pub fn pinctrl_header(&self) -> Option<&'static str> {
        Some(match self.target {
            "rp2040" => "rpi-pico-rp2040-pinctrl.h",
            "rp2350" => "rpi-pico-rp2350a-pinctrl.h",
            "rp2350b" => "rpi-pico-rp2350b-pinctrl.h",
            "esp32-s3" => "esp32s3-pinctrl.h",
            "esp32-c3" => "esp32c3-pinctrl.h",
            _ => return None,
        })
    }

    /// `&gpiob 0` for STM32 `PB0`, `&gpio0 4` elsewhere; `None` on an unknown SoC.
    pub fn gpio(&self, pin: &BusPin) -> Option<String> {
        let (low, high) = match self.family {
            Family::Stm32 => {
                let (port, n) = parse_port_pin(&pin.label)?;
                return Some(format!("&gpio{} {n}", port.to_ascii_lowercase()));
            }
            Family::Rp => ("gpio0", "gpio0_hi"),
            Family::Esp => ("gpio0", "gpio1"),
            Family::Other => return None,
        };
        let bank = if pin.number < 32 { low } else { high };
        Some(format!("&{bank} {}", pin.number % 32))
    }

    /// The ADC node label of `(adc, channel)`, where Zephyr has one.
    pub fn adc(&self, (adc, _): (u8, u8)) -> Option<String> {
        match self.family {
            Family::Stm32 => Some(format!("adc{adc}")),
            Family::Rp => Some("adc".to_string()),
            _ => None,
        }
    }

    /// The `hal_stm32` pinctrl label of an analog input (`adc1_in1_pa0`; H7 `adc1_inp16_pa0`).
    pub fn adc_pin(&self, (adc, ch): (u8, u8), pin: &BusPin) -> String {
        let input = if self.h7 { "inp" } else { "in" };
        format!("adc{adc}_{input}{ch}_{}", pin.label.to_ascii_lowercase())
    }

    /// Bits of the ADC: 16 on STM32H7, 12 on every other supported SoC.
    pub fn resolution(&self) -> u8 {
        if self.h7 {
            16
        } else {
            12
        }
    }
}
