//! Central database of microcontroller silicon targets.

/// Builds a `MuxFunction` table entry.
macro_rules! mux {
    ($sig:expr, $af:expr, $name:expr) => {
        $crate::silicon::model::MuxFunction {
            signal: $sig,
            af_number: $af,
            af_name: $name,
        }
    };
}

/// Builds a `PinSpec` for a numbered GPIO, with an
/// optional strapping requirement.
macro_rules! gpio_pin {
    ($n:expr, $name:expr, $caps:expr, $funcs:expr) => {
        $crate::silicon::model::PinSpec {
            gpio: Some($crate::silicon::model::GpioNum($n)),
            name: $name,
            capabilities: $caps,
            strapping: None,
            functions: $funcs,
        }
    };
    ($n:expr, $name:expr, $caps:expr, $funcs:expr, $strap:expr) => {
        $crate::silicon::model::PinSpec {
            gpio: Some($crate::silicon::model::GpioNum($n)),
            name: $name,
            capabilities: $caps,
            strapping: Some($strap),
            functions: $funcs,
        }
    };
}

/// Builds the RP2350 F1-F4 functions of one pin, plus the optional F11 UART
/// aux function and ADC channel. Callers import `PeripheralSignal::*`.
macro_rules! rp {
    ($spi:expr, $uart:expr, $i2c:expr, $pwm:expr $(, aux($aux:expr))? $(, adc($adc:expr))?) => {
        &[
            mux!($spi, 1, "F1"),
            mux!($uart, 2, "F2"),
            mux!($i2c, 3, "F3"),
            mux!($pwm, 4, "F4"),
            $(mux!($aux, 11, "F11"),)?
            $(mux!(Adc($adc), 0, "ADC"),)?
        ]
    };
}

pub mod esp32c3;
pub mod esp32s3;
pub mod rp2040;
mod rp2040_high;
pub mod rp2350;
pub mod rp2350b;

use super::model::SiliconTarget;

/// Resolves a microcontroller chip ID to its silicon definition.
///
/// Matching is case-insensitive and ignores `-`, `_` and spaces, so
/// `ESP32-S3`, `esp32s3` and `esp32_s3` all resolve. STM32 IDs resolve
/// through the generated tables of `crate::silicon::stm32`, RP2350 and
/// RP2354 IDs through [`rp235x`].
pub fn find_target(query: &str) -> Option<&'static SiliconTarget> {
    let clean: String = query
        .trim()
        .chars()
        .filter(|c| !matches!(c, '-' | '_' | ' '))
        .map(|c| c.to_ascii_lowercase())
        .collect();

    match clean.as_str() {
        "rp2040" => Some(&rp2040::RP2040),
        "esp32s3" => Some(&esp32s3::ESP32S3),
        "esp32c3" => Some(&esp32c3::ESP32C3),
        _ => rp235x(&clean).or_else(|| crate::silicon::stm32::find(&clean)),
    }
}

/// An RP2350 or RP2354 (same die, in-package flash) by package letter: `a`
/// for QFN-60, `b` for QFN-80, optionally followed by the package revision
/// and die stepping of the marking (`RP2350B0A2`). A bare `rp2350` is the A.
fn rp235x(clean: &str) -> Option<&'static SiliconTarget> {
    let rest = ["rp2350", "rp2354", "rp235x"]
        .iter()
        .find_map(|p| clean.strip_prefix(p))?;
    let mut chars = rest.chars();
    let package = chars.next();
    if !matches!(chars.as_str(), "" | "0a2" | "0a3" | "0a4") {
        return None;
    }
    match package {
        None | Some('a') => Some(&rp2350::RP2350),
        Some('b') => Some(&rp2350b::RP2350B),
        _ => None,
    }
}
