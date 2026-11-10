//! Part data of a `source_component`: part numbers and typed values.
//!
//! Field names follow `source_component_base.ts` and the `source_simple_*`
//! files of tscircuit/circuit-json, checked against a real tscircuit build.
//! Values are SI base units (ohms, farads, henries, volts, amps, hertz);
//! numbers sent as strings (`"10k"`, `"100nF"`) read the same.

use std::collections::BTreeMap;

use serde::Deserialize;

use super::units::quantity;

/// Everything a `source_component` says about the part it places.
///
/// [`super::Component`] keeps the identity fields; this struct carries the
/// rest and links back through `source_component_id`.
#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
#[serde(default)]
pub struct ComponentProps {
    #[serde(alias = "component_id", alias = "sourceComponentId")]
    pub source_component_id: Option<String>,
    /// Component kind, e.g. `simple_resistor`.
    #[serde(alias = "kind")]
    pub ftype: Option<String>,
    /// Reference designator, e.g. `R1`.
    pub name: Option<String>,
    pub manufacturer_part_number: Option<String>,
    /// Supplier name (`jlcpcb`, `digikey`, ...) to its part numbers.
    pub supplier_part_numbers: BTreeMap<String, Vec<String>>,
    pub display_value: Option<String>,
    pub display_name: Option<String>,
    pub symbol_display_value: Option<String>,
    pub are_pins_interchangeable: Option<bool>,
    /// Groups of pins the part joins internally.
    pub internally_connected_source_port_ids: Vec<Vec<String>>,
    pub source_group_id: Option<String>,
    /// Ohms.
    #[serde(deserialize_with = "quantity")]
    pub resistance: Option<f64>,
    pub display_resistance: Option<String>,
    /// Ohms, for a potentiometer.
    #[serde(deserialize_with = "quantity")]
    pub max_resistance: Option<f64>,
    /// Farads.
    #[serde(deserialize_with = "quantity")]
    pub capacitance: Option<f64>,
    pub display_capacitance: Option<String>,
    /// Volts a capacitor is rated for.
    #[serde(deserialize_with = "quantity")]
    pub max_voltage_rating: Option<f64>,
    /// Millimetres from the decoupled pin.
    #[serde(deserialize_with = "quantity")]
    pub max_decoupling_trace_length: Option<f64>,
    /// Henries.
    #[serde(deserialize_with = "quantity")]
    pub inductance: Option<f64>,
    pub display_inductance: Option<String>,
    /// Amps an inductor is rated for.
    #[serde(deserialize_with = "quantity")]
    pub max_current_rating: Option<f64>,
    /// Hertz, for a crystal or an AC source.
    #[serde(deserialize_with = "quantity")]
    pub frequency: Option<f64>,
    pub display_frequency: Option<String>,
    /// Farads a crystal expects.
    #[serde(deserialize_with = "quantity")]
    pub load_capacitance: Option<f64>,
    /// Volts of a voltage or power source.
    #[serde(deserialize_with = "quantity")]
    pub voltage: Option<f64>,
    /// Battery capacity, a plain number as circuit-json writes it.
    pub capacity: Option<f64>,
    /// Amps a fuse is rated for.
    #[serde(deserialize_with = "quantity")]
    pub current_rating_amps: Option<f64>,
    /// Volts a fuse is rated for.
    #[serde(deserialize_with = "quantity")]
    pub voltage_rating_volts: Option<f64>,
    /// LED colour, e.g. `red`.
    pub color: Option<String>,
    pub wavelength: Option<String>,
}
