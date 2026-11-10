//! Generated STM32 tables: token round trip and the hand-written G431 tables.

use super::*;
use crate::silicon::database::stm32 as old;
use crate::silicon::model::PeripheralSignal;

fn usb(signal: &PeripheralSignal) -> bool {
    matches!(signal, PeripheralSignal::UsbDm | PeripheralSignal::UsbDp)
}

/// USART3 and UART4 lack a bonded TX on the 32-pin package, so upstream
/// (CubeMX) drops them there while the datasheet AF table lists them.
fn uart_without_pins_on_32(signal: &PeripheralSignal) -> bool {
    use PeripheralSignal::*;
    matches!(
        signal,
        UartTx(3 | 4) | UartRx(3 | 4) | UartCts(3 | 4) | UartRts(3 | 4)
    )
}

fn same_as_old(query: &str, old: &SiliconTarget) {
    let new = find(query).unwrap_or_else(|| panic!("{query} missing"));
    let names = |t: &SiliconTarget| t.pins.iter().map(|p| p.name).collect::<Vec<_>>();
    assert_eq!(names(new), names(old), "{query} pins");
    for (n, o) in new.pins.iter().zip(old.pins) {
        let mut got: Vec<_> = n.functions.iter().filter(|f| !usb(&f.signal)).collect();
        let mut want: Vec<_> = o
            .functions
            .iter()
            .filter(|f| !(old.pins.len() == 26 && uart_without_pins_on_32(&f.signal)))
            .collect();
        got.sort_by_key(|f| (f.signal.name(), f.af_number));
        want.sort_by_key(|f| (f.signal.name(), f.af_number));
        assert_eq!(got, want, "{query} {}", n.name);
        assert_eq!(n.capabilities, o.capabilities, "{query} {}", n.name);
    }
}

#[test]
fn generated_g431_matches_the_datasheet_tables() {
    same_as_old("stm32g431", &old::STM32G431);
    same_as_old("stm32g431kb", &old::STM32G431K);
    same_as_old("stm32g431cbt", &old::STM32G431C);
    same_as_old("stm32g431cb", &old::STM32G431C);
    same_as_old("stm32g431cbu", &old::STM32G431CU);
    same_as_old("stm32g431cby", &old::STM32G431CY);
    same_as_old("stm32g431rb", &old::STM32G431R);
    same_as_old("stm32g431mb", &old::STM32G431M);
    same_as_old("stm32g431vb", &old::STM32G431V);
}

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
fn rejects_queries_shorter_than_a_line_or_unknown() {
    assert!(find("stm32g4").is_none());
    assert!(find("stm32g431cbx").is_none());
    assert!(find("stm32g431gb").is_none());
    assert!(find("stm32u575").is_none());
}
