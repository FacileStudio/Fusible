//! Renderer snapshots over one STM32G431 board: I2C sensor, SPI flash, GPS UART,
//! a divided battery sense input, an LED and a button.
//!
//! The expected files live in `src/codegen/tests/snapshots/`. Run with
//! `FUSIBLE_BLESS=1` to rewrite them after an intended change.

use std::path::{Path, PathBuf};

use crate::codegen::model::IndicatorKind;
use crate::codegen::{
    board_bindings, c, extract_mcu_pins, generate_pins_rs, rust, zephyr, BoardBindings,
};
use crate::netlist::Netlist;
use crate::parser;

fn dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src/codegen/tests/snapshots")
}

/// The fixture board on `chip`, each STM32 pin label renamed by `pins`.
pub(super) fn board_on(chip: &str, pins: &[(&str, &str)]) -> BoardBindings {
    let path = dir().join("stm32_board.json");
    let mut source = std::fs::read_to_string(&path).unwrap();
    for (from, to) in pins {
        source = source.replace(
            &format!("\"name\": \"{from}\""),
            &format!("\"name\": \"{to}\""),
        );
    }
    let circuit = parser::parse(&source, &path).unwrap();
    let netlist = Netlist::from_circuit(&circuit);
    let mcu = extract_mcu_pins(&circuit, &netlist, Some(chip)).unwrap();
    board_bindings("sensor-node", mcu, &circuit, &netlist)
}

fn bindings() -> BoardBindings {
    board_on("stm32g431cbu", &[])
}

pub(super) fn snapshot(name: &str, actual: &str) {
    let path = dir().join(name);
    if std::env::var("FUSIBLE_BLESS").as_deref() == Ok("1") {
        std::fs::write(&path, actual).unwrap();
    }
    let expected = std::fs::read_to_string(&path).unwrap_or_default();
    assert_eq!(
        actual, expected,
        "{name} differs; rerun with FUSIBLE_BLESS=1 if intended"
    );
}

#[test]
fn the_model_finds_every_bus_and_the_divider() {
    let b = bindings();
    assert_eq!(b.i2c.len(), 1);
    assert_eq!(b.i2c[0].instance, Some(1));
    assert_eq!(b.i2c[0].devices, ["U2"]);
    assert_eq!(b.spi[0].instance, Some(1));
    assert_eq!(b.spi[0].chip_selects[0].device.as_deref(), Some("U3"));
    assert_eq!(b.uart[0].instance, Some(1));
    assert_eq!(b.uart[0].tx.af, Some(7));
    let ratio = b.adc[0].divider.unwrap().ratio();
    assert!((ratio - 147.0 / 47.0).abs() < 1e-9, "{ratio}");
    assert_eq!(b.adc[0].channel, Some((1, 1)));
    assert_eq!(
        b.gpio.iter().map(|p| p.signal.as_str()).collect::<Vec<_>>(),
        ["BUTTON", "LED"]
    );
    let found: Vec<_> = b
        .indicators
        .iter()
        .map(|i| (i.kind, i.device.as_str(), i.active_low))
        .collect();
    assert_eq!(
        found,
        [
            (IndicatorKind::Button, "SW1", true),
            (IndicatorKind::Led, "D1", false)
        ]
    );
}

#[test]
fn renderers_match_their_snapshots() {
    let b = bindings();
    snapshot("pins.rs.txt", &generate_pins_rs(&b.mcu, &b.board));
    snapshot("board.rs.txt", &rust::board_rs(&b).unwrap());
    snapshot("pins.h.txt", &c::pins_h(&b));
    snapshot(
        "app.overlay.txt",
        &zephyr::overlay(&b, &Default::default()).text,
    );
}

#[test]
fn an_esp32_board_rs_owns_gpio_singletons() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/commands/init/templates/esp32-s3/circuit.json");
    let source = std::fs::read_to_string(&path).unwrap();
    let circuit = parser::parse(&source, &path).unwrap();
    let netlist = Netlist::from_circuit(&circuit);
    let mcu = extract_mcu_pins(&circuit, &netlist, Some("esp32-s3")).unwrap();
    let board = rust::board_rs(&board_bindings("demo", mcu, &circuit, &netlist)).unwrap();

    assert!(
        board.contains("use esp_hal::peripherals as hal;"),
        "{board}"
    );
    assert!(board.contains("pub led: hal::GPIO48<'static>,"), "{board}");
    assert!(board.contains("            led: $p.GPIO48,"), "{board}");
    assert!(
        board.contains("pub fn init(config: esp_hal::Config) -> Board {"),
        "{board}"
    );
}
