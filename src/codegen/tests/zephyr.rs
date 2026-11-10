//! Zephyr overlays of the fixture board moved onto each supported family, and
//! the manifest overrides and fail-closed paths of device nodes.

use super::snapshots::{board_on, snapshot};
use crate::codegen::zephyr::{overlay, Overrides, ZephyrDevice};

const RP2040: &[(&str, &str)] = &[
    ("PB8", "GPIO5"),
    ("PB9", "GPIO4"),
    ("PA5", "GPIO18"),
    ("PA6", "GPIO16"),
    ("PA7", "GPIO19"),
    ("PA4", "GPIO17"),
    ("PA9", "GPIO0"),
    ("PA10", "GPIO1"),
    ("PA0", "GPIO26"),
    ("PB0", "GPIO25"),
    ("PC13", "GPIO2"),
];

const RP2350B: &[(&str, &str)] = &[
    ("PB8", "GPIO31"),
    ("PB9", "GPIO30"),
    ("PA5", "GPIO34"),
    ("PA6", "GPIO32"),
    ("PA7", "GPIO35"),
    ("PA4", "GPIO33"),
    ("PA9", "GPIO36"),
    ("PA10", "GPIO37"),
    ("PA0", "GPIO47"),
    ("PB0", "GPIO45"),
    ("PC13", "GPIO2"),
];

const ESP32S3: &[(&str, &str)] = &[
    ("PB8", "GPIO2"),
    ("PB9", "GPIO1"),
    ("PA5", "GPIO12"),
    ("PA6", "GPIO13"),
    ("PA7", "GPIO11"),
    ("PA4", "GPIO10"),
    ("PA9", "GPIO43"),
    ("PA10", "GPIO44"),
    ("PA0", "GPIO4"),
    ("PB0", "GPIO38"),
    ("PC13", "GPIO0"),
];

const ESP32C3: &[(&str, &str)] = &[
    ("PB8", "GPIO7"),
    ("PB9", "GPIO6"),
    ("PA5", "GPIO10"),
    ("PA6", "GPIO8"),
    ("PA7", "GPIO11"),
    ("PA4", "GPIO5"),
    ("PA9", "GPIO21"),
    ("PA10", "GPIO20"),
    ("PA0", "GPIO1"),
    ("PB0", "GPIO4"),
    ("PC13", "GPIO9"),
];

fn text(chip: &str, pins: &[(&str, &str)]) -> String {
    overlay(&board_on(chip, pins), &Overrides::new()).text
}

#[test]
fn each_family_matches_its_snapshot() {
    snapshot("app.rp2040.overlay.txt", &text("rp2040", RP2040));
    snapshot("app.rp2350a.overlay.txt", &text("rp2350a", RP2040));
    snapshot("app.rp2350b.overlay.txt", &text("rp2350b", RP2350B));
    snapshot("app.esp32s3.overlay.txt", &text("esp32-s3", ESP32S3));
    snapshot("app.esp32c3.overlay.txt", &text("esp32-c3", ESP32C3));
}

#[test]
fn the_manifest_names_a_part_the_table_does_not_know() {
    let flash = ZephyrDevice {
        compatible: Some("jedec,spi-nor".to_string()),
        spi_max_frequency: Some(8_000_000),
        ..Default::default()
    };
    let typo = ZephyrDevice::default();
    let overrides = Overrides::from([("U3".to_string(), flash), ("U9".to_string(), typo)]);
    let out = overlay(&board_on("stm32g431cbu", &[]), &overrides);
    assert!(out
        .text
        .contains("u3: u3@0 {\n\t\tcompatible = \"jedec,spi-nor\";"));
    assert!(out.text.contains("spi-max-frequency = <8000000>;"));
    assert_eq!(out.todo.len(), 1, "{:?}", out.todo);
    assert!(out.todo[0].contains("zephyr.U9` names no device"));
}

#[test]
fn an_untied_strap_pin_leaves_the_address_to_the_user() {
    let out = overlay(
        &board_on("stm32g431cbu", &[("SDO", "X")]),
        &Overrides::new(),
    );
    assert!(out.todo.iter().any(|t| t.contains("0x76 or 0x77")));
    assert!(out.text.contains("u2: u2 {\n\t\tstatus = \"disabled\";"));
    assert!(!out.text.contains("bosch,bme280"));
}

#[test]
fn a_pull_resistor_to_ground_does_not_reach_the_led_beyond_it() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/commands/init/templates/stm32g4/circuit.json");
    let source = std::fs::read_to_string(&path).unwrap();
    let circuit = crate::parser::parse(&source, &path).unwrap();
    let netlist = crate::netlist::Netlist::from_circuit(&circuit);
    let mcu = crate::codegen::extract_mcu_pins(&circuit, &netlist, Some("stm32g431kbt")).unwrap();
    let b = crate::codegen::board_bindings("demo", mcu, &circuit, &netlist);
    let leds: Vec<_> = b
        .indicators
        .iter()
        .map(|i| (i.device.as_str(), i.pin.label.as_str(), i.active_low))
        .collect();
    assert_eq!(leds, [("D1", "PA5", false)]);
}

#[test]
fn an_unknown_soc_gets_no_guessed_controller() {
    let out = overlay(&board_on("U1", &[]), &Overrides::new());
    assert!(!out.text.contains("&i2c_bus0"));
    assert!(!out.text.contains("pinctrl-0"));
    assert!(out.todo.iter().any(|t| t.starts_with("I2C_BUS0")));
}
