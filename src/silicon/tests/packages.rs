//! Package variants: STM32G431 48-pin against DS12589 Rev 6, Table 12; RP2350A/B; ESP32-C3.

use crate::silicon::*;

#[test]
fn c_package_variants_add_their_extra_gpios() {
    let bonded = |chip: &str, pin: &str| find_target(chip).unwrap().find_pin(pin).is_some();
    assert!(bonded("stm32g431cbu", "PC10"));
    assert!(bonded("stm32g431cbu", "PC4"));
    assert!(!bonded("stm32g431cby", "PC10"));
    assert!(bonded("stm32g431cby", "PC11"));
    assert!(!bonded("stm32g431cbt", "PC11"));
    assert!(find_target("stm32g431cbx").is_none());
}

/// RP2040 datasheet Table 279: GPIO26-29 keep F1 SPI1 and F2/F3 next to the ADC.
#[test]
fn rp2040_adc_pins_keep_their_spi_uart_and_i2c_functions() {
    use PeripheralSignal::*;
    let chip = find_target("rp2040").unwrap();
    let rows = [
        ("GPIO26", [SpiSck(1), UartCts(1), I2cSda(1), Pwm(5, 'A')]),
        ("GPIO27", [SpiMosi(1), UartRts(1), I2cScl(1), Pwm(5, 'B')]),
        ("GPIO28", [SpiMiso(1), UartTx(0), I2cSda(0), Pwm(6, 'A')]),
        ("GPIO29", [SpiCs(1, 0), UartRx(0), I2cScl(0), Pwm(6, 'B')]),
    ];
    for (name, signals) in rows {
        let pin = chip.find_pin(name).unwrap();
        for (af, signal) in (1..).zip(signals) {
            assert_eq!(
                pin.get_af(&signal).map(|f| f.af_number),
                Some(af),
                "{name} {signal:?}"
            );
        }
    }
}

#[test]
fn rp2350_follows_the_rp2040_matrix_with_uart_aux() {
    let chip = find_target("RP2350A").unwrap();
    let gp2 = chip.find_pin("GPIO2").unwrap();
    assert_eq!(
        gp2.get_af(&PeripheralSignal::UartTx(0)).unwrap().af_number,
        11
    );
    assert!(chip
        .find_pin("GPIO23")
        .unwrap()
        .supports_signal(&PeripheralSignal::SpiMosi(0)));
    assert!(chip
        .find_pin("GPIO29")
        .unwrap()
        .supports_signal(&PeripheralSignal::Adc(3)));
    assert_eq!(chip.pins.len(), 30);
}

#[test]
fn rp2350b_bonds_gpio0_to_47_with_the_adc_on_gpio40_to_47() {
    let chip = find_target("RP2350B").unwrap();
    assert_eq!(chip.pins.len(), 48);
    let pin = |n: u8| chip.find_pin(&format!("GPIO{n}")).unwrap();
    assert!(pin(47).supports_signal(&PeripheralSignal::Adc(7)));
    assert!(pin(40).supports_signal(&PeripheralSignal::Adc(0)));
    assert!(!pin(26).capabilities.is_analog);
    assert!(pin(36).supports_signal(&PeripheralSignal::Pwm(10, 'A')));
    assert_eq!(
        pin(46)
            .get_af(&PeripheralSignal::UartTx(0))
            .unwrap()
            .af_number,
        11
    );
    assert!(pin(33).supports_signal(&PeripheralSignal::SpiCs(0, 0)));
}

#[test]
fn rp235x_ids_resolve_by_package_letter() {
    let name = |id: &str| find_target(id).map(|t| t.name);
    assert_eq!(name("rp2350"), Some("rp2350"));
    assert_eq!(name("RP2350A0A2"), Some("rp2350"));
    assert_eq!(name("rp2354b"), Some("rp2350b"));
    assert_eq!(name("RP2350B0A4"), Some("rp2350b"));
    assert_eq!(name("rp2350c"), None);
    assert_eq!(name("rp2350b0a9"), None);
    assert!(find_target("rp2350a").unwrap().find_pin("GPIO30").is_none());
}

#[test]
fn esp32c3_reserves_flash_and_straps_gpio9() {
    let chip = find_target("esp32-c3").unwrap();
    assert!(
        chip.find_pin("GPIO14")
            .unwrap()
            .capabilities
            .is_reserved_flash
    );
    assert!(chip.find_pin("GPIO9").unwrap().strapping.is_some());
    assert!(chip.find_pin("GPIO2").unwrap().capabilities.is_analog);
    assert!(chip.find_pin("GPIO22").is_none());
}
