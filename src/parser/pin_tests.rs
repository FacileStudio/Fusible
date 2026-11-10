//! Tests for pin attributes flattened into `source_port`, and quantity reading.

use std::path::Path;

use super::units::si_value;
use super::*;

const FIXTURE: &str = include_str!("fixtures/pin_attributes.json");

fn port<'a>(circuit: &'a Circuit, name: &str) -> &'a Port {
    let mut ports = circuit.ports();
    ports
        .find(|p| p.name.as_deref() == Some(name))
        .expect("port in fixture")
}

#[test]
fn chip_pin_attributes_and_no_connect_read_back() {
    let circuit = parse(FIXTURE, Path::new("pins.json")).expect("fixture parses");

    let vcc = port(&circuit, "VCC");
    assert_eq!(vcc.port_id.as_deref(), Some("source_port_0"));
    assert_eq!(vcc.pin_number, Some(1));
    assert_eq!(vcc.attributes.requires_power, Some(true));
    assert_eq!(vcc.attributes.requires_voltage, Some(3.3));
    assert_eq!(vcc.attributes.required_voltage_tolerance, Some(0.05));
    assert_eq!(vcc.attributes.should_have_decoupling_capacitor, Some(true));
    let cap = vcc.attributes.recommended_decoupling_capacitor_capacitance;
    assert!((cap.expect("capacitance") - 100e-9).abs() < 1e-15);
    assert_eq!(vcc.attributes.do_not_connect, None);

    assert_eq!(port(&circuit, "GND").attributes.requires_ground, Some(true));
    assert_eq!(port(&circuit, "NC").attributes.do_not_connect, Some(true));
}

#[test]
fn camel_case_aliases_still_read_beside_flattened_attributes() {
    let circuit = parse(FIXTURE, Path::new("pins.json")).expect("fixture parses");

    let sda = port(&circuit, "SDA");
    assert_eq!(sda.port_id.as_deref(), Some("source_port_3"));
    assert_eq!(sda.pin_number, Some(4));
    assert_eq!(sda.attributes.is_using_open_drain, Some(true));
    assert_eq!(sda.attributes.needs_external_pullup, Some(true));
    assert_eq!(sda.attributes.provides_voltage, Some(1.8));
    assert!(sda.attributes.unreadable.is_none());
}

#[test]
fn a_malformed_attribute_keeps_the_port_and_is_reported() {
    let circuit = parse(FIXTURE, Path::new("pins.json")).expect("still parses");

    let en = port(&circuit, "EN");
    assert_eq!(en.pin_number, Some(5));
    assert_eq!(en.attributes.must_be_connected, None);
    assert!(en.attributes.unreadable.is_some());
    let model = circuit.model();
    assert_eq!(model.unreadable.len(), 1, "{:?}", model.unreadable);
    assert!(model.unreadable[0].starts_with("source_port_4: source_port:"));
}

#[test]
fn duplicate_spellings_stay_a_hard_error() {
    let json = r#"[{"type": "source_port", "port_id": "a", "source_port_id": "b"}]"#;
    assert!(parse(json, Path::new("pins.json")).is_err());
}

#[test]
fn quantities_read_numbers_with_si_prefixes_and_units() {
    assert_eq!(si_value("3.3V"), Some(3.3));
    assert_eq!(si_value(" 5 "), Some(5.0));
    assert_eq!(si_value("10k"), Some(10_000.0));
    assert_eq!(si_value("4.7kΩ"), Some(4700.0));
    assert_eq!(si_value("1e-7"), Some(1e-7));
    assert!((si_value("100 nF").expect("nF") - 1e-7).abs() < 1e-18);
    assert!((si_value("2.2µF").expect("µF") - 2.2e-6).abs() < 1e-18);
    assert_eq!(si_value("1.8-3.6V"), None);
    assert_eq!(si_value("about 3V"), None);
    assert_eq!(si_value("inf"), None);
    assert_eq!(si_value(""), None);
}
