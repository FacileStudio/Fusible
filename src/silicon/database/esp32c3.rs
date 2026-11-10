//! Espressif ESP32-C3 GPIO, strapping, and reserved flash pin tables.
//!
//! Source: ESP32-C3 datasheet v2.x, chapter 2 "Pins" and 3.3 "Strapping
//! pins". The GPIO matrix routes UART, SPI, I2C and LEDC to any GPIO, so no
//! pin lists them; GPIO0-4 list ADC1_CH0-4 and GPIO5 ADC2_CH0 (Table 2-6
//! "Analog Functions"); GPIO12-17 drive the in-package SPI flash; GPIO9
//! selects SPI boot when high at reset.

use crate::silicon::model::PeripheralSignal::AdcChannel;
use crate::silicon::model::{PinCapabilities, PinSpec, SiliconTarget, StrappingRequirement};

const GPIO: PinCapabilities = PinCapabilities {
    is_gpio: true,
    is_analog: false,
    is_input_only: false,
    is_reserved_flash: false,
};

const ADC: PinCapabilities = PinCapabilities {
    is_analog: true,
    ..GPIO
};

const FLASH: PinCapabilities = PinCapabilities {
    is_gpio: false,
    is_reserved_flash: true,
    ..GPIO
};

const BOOT: StrappingRequirement = StrappingRequirement::MustBeHigh {
    purpose: "SPI flash boot mode",
    violation_consequence: "holding GPIO9 LOW at reset enters the ROM download bootloader",
};

static ESP32C3_PINS: &[PinSpec] = &[
    gpio_pin!(0, "GPIO0", ADC, &[mux!(AdcChannel(1, 0), 0, "ADC1_CH0")]),
    gpio_pin!(1, "GPIO1", ADC, &[mux!(AdcChannel(1, 1), 0, "ADC1_CH1")]),
    gpio_pin!(2, "GPIO2", ADC, &[mux!(AdcChannel(1, 2), 0, "ADC1_CH2")]),
    gpio_pin!(3, "GPIO3", ADC, &[mux!(AdcChannel(1, 3), 0, "ADC1_CH3")]),
    gpio_pin!(4, "GPIO4", ADC, &[mux!(AdcChannel(1, 4), 0, "ADC1_CH4")]),
    gpio_pin!(5, "GPIO5", ADC, &[mux!(AdcChannel(2, 0), 0, "ADC2_CH0")]),
    gpio_pin!(6, "GPIO6", GPIO, &[]),
    gpio_pin!(7, "GPIO7", GPIO, &[]),
    gpio_pin!(8, "GPIO8", GPIO, &[]),
    gpio_pin!(9, "GPIO9", GPIO, &[], BOOT),
    gpio_pin!(10, "GPIO10", GPIO, &[]),
    gpio_pin!(11, "GPIO11", GPIO, &[]),
    gpio_pin!(12, "GPIO12", FLASH, &[]),
    gpio_pin!(13, "GPIO13", FLASH, &[]),
    gpio_pin!(14, "GPIO14", FLASH, &[]),
    gpio_pin!(15, "GPIO15", FLASH, &[]),
    gpio_pin!(16, "GPIO16", FLASH, &[]),
    gpio_pin!(17, "GPIO17", FLASH, &[]),
    gpio_pin!(18, "GPIO18", GPIO, &[]),
    gpio_pin!(19, "GPIO19", GPIO, &[]),
    gpio_pin!(20, "GPIO20", GPIO, &[]),
    gpio_pin!(21, "GPIO21", GPIO, &[]),
];

pub static ESP32C3: SiliconTarget = SiliconTarget {
    name: "esp32-c3",
    family: "Espressif",
    pins: ESP32C3_PINS,
};
