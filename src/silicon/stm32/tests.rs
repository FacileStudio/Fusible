//! Generated STM32 tables: token round trip, one chip per family, queries.
//!
//! The STM32G431 tables were checked equal to the hand-written DS12589
//! tables they replaced, with two upstream differences: USB DM/DP on
//! PA11/PA12 (the old table omitted them), and no USART3/UART4 on the
//! 32-pin package, whose TX pins are not bonded.

use super::*;
use crate::silicon::model::PeripheralSignal;

#[test]
fn every_token_of_every_family_parses() {
    for (key, text) in SOURCES {
        for line in text.lines().filter(|l| !l.starts_with(['#', 't', 'p'])) {
            for token in line.split(' ').skip(1) {
                assert!(parse::function(token).is_some(), "{key}: {token}");
            }
        }
    }
}

#[test]
fn resolves_one_chip_of_each_family() {
    for (chip, pin, signal, af) in [
        ("stm32g071rb", "PA9", PeripheralSignal::UartTx(1), 1),
        ("stm32f411ceu6", "PA9", PeripheralSignal::UartTx(1), 7),
        ("stm32l476rg", "PB6", PeripheralSignal::I2cScl(1), 4),
        ("stm32h743zi", "PD8", PeripheralSignal::UartTx(3), 7),
        ("stm32g474re", "PA14", PeripheralSignal::SwdClk, 0),
    ] {
        let target = find(chip).unwrap_or_else(|| panic!("{chip} missing"));
        let found = target.find_pin(pin).and_then(|p| p.get_af(&signal));
        assert_eq!(found.map(|f| f.af_number), Some(af), "{chip} {pin}");
    }
}

#[test]
fn drops_uarts_without_bonded_pins_on_the_32_pin_package() {
    let pb8 = |chip: &str| find(chip).and_then(|t| t.find_pin("PB8"));
    let k = pb8("stm32g431kb").unwrap();
    assert!(!k.supports_signal(&PeripheralSignal::UartRx(3)));
    assert!(pb8("stm32g431rb")
        .unwrap()
        .supports_signal(&PeripheralSignal::UartRx(3)));
}

#[test]
fn rejects_queries_shorter_than_a_line_or_unknown() {
    assert!(find("stm32g4").is_none());
    assert!(find("stm32g431cbx").is_none());
    assert!(find("stm32g431gb").is_none());
    assert!(find("stm32u575").is_none());
}
