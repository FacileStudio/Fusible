//! STM32G431 48-pin package variants against DS12589 Rev 6, Table 12.

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
