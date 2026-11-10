//! Raspberry Pi RP2350A (QFN-60, GPIO0-29) Function Select and ADC tables.
//!
//! Source: RP2350 datasheet, section 9.4 "Function select". F1 to F4 follow
//! the RP2040 pattern; F11 adds the UART TX/RX "aux" function on each CTS
//! and RTS pin. ADC0-3 sit on GPIO26-29. RP2350B (48 GPIOs) is not listed.

use crate::silicon::model::{PinCapabilities, PinSpec, SiliconTarget};
use crate::silicon::PeripheralSignal::*;

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

/// Builds the F1-F4 functions of one pin, plus the optional F11 UART aux
/// function and ADC channel.
macro_rules! rp {
    ($spi:expr, $uart:expr, $i2c:expr, $pwm:expr $(, aux($aux:expr))? $(, adc($adc:expr))?) => {
        &[
            mux!($spi, 1, "F1"),
            mux!($uart, 2, "F2"),
            mux!($i2c, 3, "F3"),
            mux!($pwm, 4, "F4"),
            $(mux!($aux, 11, "F11"),)?
            $(mux!(Adc($adc), 0, "ADC"),)?
        ]
    };
}

#[rustfmt::skip]
static RP2350_PINS: &[PinSpec] = &[
    gpio_pin!(0, "GPIO0", GPIO, rp!(SpiMiso(0), UartTx(0), I2cSda(0), Pwm(0, 'A'))),
    gpio_pin!(1, "GPIO1", GPIO, rp!(SpiCs(0, 0), UartRx(0), I2cScl(0), Pwm(0, 'B'))),
    gpio_pin!(2, "GPIO2", GPIO, rp!(SpiSck(0), UartCts(0), I2cSda(1), Pwm(1, 'A'), aux(UartTx(0)))),
    gpio_pin!(3, "GPIO3", GPIO, rp!(SpiMosi(0), UartRts(0), I2cScl(1), Pwm(1, 'B'), aux(UartRx(0)))),
    gpio_pin!(4, "GPIO4", GPIO, rp!(SpiMiso(0), UartTx(1), I2cSda(0), Pwm(2, 'A'))),
    gpio_pin!(5, "GPIO5", GPIO, rp!(SpiCs(0, 0), UartRx(1), I2cScl(0), Pwm(2, 'B'))),
    gpio_pin!(6, "GPIO6", GPIO, rp!(SpiSck(0), UartCts(1), I2cSda(1), Pwm(3, 'A'), aux(UartTx(1)))),
    gpio_pin!(7, "GPIO7", GPIO, rp!(SpiMosi(0), UartRts(1), I2cScl(1), Pwm(3, 'B'), aux(UartRx(1)))),
    gpio_pin!(8, "GPIO8", GPIO, rp!(SpiMiso(1), UartTx(1), I2cSda(0), Pwm(4, 'A'))),
    gpio_pin!(9, "GPIO9", GPIO, rp!(SpiCs(1, 0), UartRx(1), I2cScl(0), Pwm(4, 'B'))),
    gpio_pin!(10, "GPIO10", GPIO, rp!(SpiSck(1), UartCts(1), I2cSda(1), Pwm(5, 'A'), aux(UartTx(1)))),
    gpio_pin!(11, "GPIO11", GPIO, rp!(SpiMosi(1), UartRts(1), I2cScl(1), Pwm(5, 'B'), aux(UartRx(1)))),
    gpio_pin!(12, "GPIO12", GPIO, rp!(SpiMiso(1), UartTx(0), I2cSda(0), Pwm(6, 'A'))),
    gpio_pin!(13, "GPIO13", GPIO, rp!(SpiCs(1, 0), UartRx(0), I2cScl(0), Pwm(6, 'B'))),
    gpio_pin!(14, "GPIO14", GPIO, rp!(SpiSck(1), UartCts(0), I2cSda(1), Pwm(7, 'A'), aux(UartTx(0)))),
    gpio_pin!(15, "GPIO15", GPIO, rp!(SpiMosi(1), UartRts(0), I2cScl(1), Pwm(7, 'B'), aux(UartRx(0)))),
    gpio_pin!(16, "GPIO16", GPIO, rp!(SpiMiso(0), UartTx(0), I2cSda(0), Pwm(0, 'A'))),
    gpio_pin!(17, "GPIO17", GPIO, rp!(SpiCs(0, 0), UartRx(0), I2cScl(0), Pwm(0, 'B'))),
    gpio_pin!(18, "GPIO18", GPIO, rp!(SpiSck(0), UartCts(0), I2cSda(1), Pwm(1, 'A'), aux(UartTx(0)))),
    gpio_pin!(19, "GPIO19", GPIO, rp!(SpiMosi(0), UartRts(0), I2cScl(1), Pwm(1, 'B'), aux(UartRx(0)))),
    gpio_pin!(20, "GPIO20", GPIO, rp!(SpiMiso(0), UartTx(1), I2cSda(0), Pwm(2, 'A'))),
    gpio_pin!(21, "GPIO21", GPIO, rp!(SpiCs(0, 0), UartRx(1), I2cScl(0), Pwm(2, 'B'))),
    gpio_pin!(22, "GPIO22", GPIO, rp!(SpiSck(0), UartCts(1), I2cSda(1), Pwm(3, 'A'), aux(UartTx(1)))),
    gpio_pin!(23, "GPIO23", GPIO, rp!(SpiMosi(0), UartRts(1), I2cScl(1), Pwm(3, 'B'), aux(UartRx(1)))),
    gpio_pin!(24, "GPIO24", GPIO, rp!(SpiMiso(1), UartTx(1), I2cSda(0), Pwm(4, 'A'))),
    gpio_pin!(25, "GPIO25", GPIO, rp!(SpiCs(1, 0), UartRx(1), I2cScl(0), Pwm(4, 'B'))),
    gpio_pin!(26, "GPIO26", ADC, rp!(SpiSck(1), UartCts(1), I2cSda(1), Pwm(5, 'A'), aux(UartTx(1)), adc(0))),
    gpio_pin!(27, "GPIO27", ADC, rp!(SpiMosi(1), UartRts(1), I2cScl(1), Pwm(5, 'B'), aux(UartRx(1)), adc(1))),
    gpio_pin!(28, "GPIO28", ADC, rp!(SpiMiso(1), UartTx(0), I2cSda(0), Pwm(6, 'A'), adc(2))),
    gpio_pin!(29, "GPIO29", ADC, rp!(SpiCs(1, 0), UartRx(0), I2cScl(0), Pwm(6, 'B'), adc(3))),
];

pub static RP2350: SiliconTarget = SiliconTarget {
    name: "rp2350",
    family: "Raspberry Pi",
    pins: RP2350_PINS,
};
