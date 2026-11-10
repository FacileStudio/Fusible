//! Declared electrical intent of a pin: circuit-json `source_pin_attributes`.
//!
//! tscircuit writes these on every `source_port` from `<chip pinAttributes>`
//! and `noConnect`. Field names are the snake_case keys of
//! `src/source/properties/source_pin_attributes.ts` in tscircuit/circuit-json.
//! Every field is optional: `None` means the design declares nothing, never
//! "false".

use serde::{Deserialize, Deserializer};
use serde_json::{Map, Value};

use super::units::quantity;

/// The pin attributes flattened into a `source_port`.
///
/// Voltages arrive as numbers or strings (`"3.3V"`) and read as volts.
/// When a value has the wrong type, every field stays `None` and
/// [`PinAttributes::unreadable`] holds the serde error, so a rule can report
/// the port instead of trusting an empty declaration.
#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
#[serde(default)]
pub struct PinAttributes {
    pub is_input: Option<bool>,
    pub is_output: Option<bool>,
    pub is_bidirectional: Option<bool>,
    pub is_passive: Option<bool>,
    pub can_use_tri_state: Option<bool>,
    pub is_using_tri_state: Option<bool>,
    pub can_use_open_collector: Option<bool>,
    pub is_using_open_collector: Option<bool>,
    pub can_use_open_emitter: Option<bool>,
    pub is_using_open_emitter: Option<bool>,
    pub is_gpio: Option<bool>,
    pub highlight_color: Option<String>,
    pub must_be_connected: Option<bool>,
    pub provides_power: Option<bool>,
    pub requires_power: Option<bool>,
    pub provides_ground: Option<bool>,
    pub requires_ground: Option<bool>,
    /// Volts this pin supplies.
    #[serde(deserialize_with = "quantity")]
    pub provides_voltage: Option<f64>,
    /// Volts this pin needs.
    #[serde(deserialize_with = "quantity")]
    pub requires_voltage: Option<f64>,
    /// Allowed relative deviation from `requires_voltage`, 0.05 for 5 %.
    pub required_voltage_tolerance: Option<f64>,
    pub do_not_connect: Option<bool>,
    pub include_in_board_pinout: Option<bool>,
    pub can_use_internal_pullup: Option<bool>,
    pub is_using_internal_pullup: Option<bool>,
    pub needs_external_pullup: Option<bool>,
    pub can_use_internal_pulldown: Option<bool>,
    pub is_using_internal_pulldown: Option<bool>,
    pub needs_external_pulldown: Option<bool>,
    pub can_use_open_drain: Option<bool>,
    pub is_using_open_drain: Option<bool>,
    pub can_use_push_pull: Option<bool>,
    pub is_using_push_pull: Option<bool>,
    pub should_have_decoupling_capacitor: Option<bool>,
    /// Farads, from a number or a string such as `"100nF"`.
    #[serde(deserialize_with = "quantity")]
    pub recommended_decoupling_capacitor_capacitance: Option<f64>,
    pub is_configured_for_i2c_sda: Option<bool>,
    pub is_configured_for_i2c_scl: Option<bool>,
    pub is_configured_for_spi_mosi: Option<bool>,
    pub is_configured_for_spi_miso: Option<bool>,
    pub is_configured_for_spi_sck: Option<bool>,
    pub is_configured_for_spi_cs: Option<bool>,
    pub is_configured_for_uart_tx: Option<bool>,
    pub is_configured_for_uart_rx: Option<bool>,
    pub supports_i2c_sda: Option<bool>,
    pub supports_i2c_scl: Option<bool>,
    pub supports_spi_mosi: Option<bool>,
    pub supports_spi_miso: Option<bool>,
    pub supports_spi_sck: Option<bool>,
    pub supports_spi_cs: Option<bool>,
    pub supports_uart_tx: Option<bool>,
    pub supports_uart_rx: Option<bool>,
    /// Why the attributes could not be read, when a value had the wrong type.
    #[serde(skip)]
    pub unreadable: Option<String>,
}

/// Reads the attributes from the keys a `source_port` leaves over.
///
/// A malformed attribute never rejects the port, so `parse` accepts what it
/// accepted before attributes were modelled; the error lands in
/// [`PinAttributes::unreadable`] instead.
pub(super) fn lenient<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<PinAttributes, D::Error> {
    let rest = Map::<String, Value>::deserialize(deserializer)?;
    Ok(
        PinAttributes::deserialize(Value::Object(rest)).unwrap_or_else(|err| PinAttributes {
            unreadable: Some(err.to_string()),
            ..PinAttributes::default()
        }),
    )
}
