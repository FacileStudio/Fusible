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

pub mod esp32c3;
pub mod esp32s3;
pub mod rp2040;
pub mod rp2350;

use super::model::SiliconTarget;

/// Resolves a microcontroller chip ID to its silicon definition.
///
/// Matching is case-insensitive and ignores `-`, `_` and spaces, so
/// `ESP32-S3`, `esp32s3` and `esp32_s3` all resolve. STM32 IDs resolve
/// through the generated tables of `crate::silicon::stm32`.
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
        "rp2350" | "rp2350a" | "rp235xa" => Some(&rp2350::RP2350),
        _ => crate::silicon::stm32::find(&clean),
    }
}
