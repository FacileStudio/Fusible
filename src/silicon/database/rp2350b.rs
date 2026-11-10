//! Raspberry Pi RP2350B (QFN-80, GPIO0-47) Function Select and ADC tables.
//!
//! Source: RP2350 datasheet, section 9.4 "Function select", Table 677, rows
//! 0-47 (30-47 are QFN-80 only), and section 9.2: ADC0-7 sit on GPIO40-47,
//! so GPIO26-29 have no analog input on this package. Cross-checked against
//! the pico-sdk `io_bank0.h` FUNCSEL values. PWM slices 8-11 exist only here.

use super::rp2350::{ADC, GPIO};
use crate::silicon::model::{PinSpec, SiliconTarget};
use crate::silicon::PeripheralSignal::*;

#[rustfmt::skip]
static RP2350B_PINS: &[PinSpec] = &[
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
    gpio_pin!(26, "GPIO26", GPIO, rp!(SpiSck(1), UartCts(1), I2cSda(1), Pwm(5, 'A'), aux(UartTx(1)))),
    gpio_pin!(27, "GPIO27", GPIO, rp!(SpiMosi(1), UartRts(1), I2cScl(1), Pwm(5, 'B'), aux(UartRx(1)))),
    gpio_pin!(28, "GPIO28", GPIO, rp!(SpiMiso(1), UartTx(0), I2cSda(0), Pwm(6, 'A'))),
    gpio_pin!(29, "GPIO29", GPIO, rp!(SpiCs(1, 0), UartRx(0), I2cScl(0), Pwm(6, 'B'))),
    gpio_pin!(30, "GPIO30", GPIO, rp!(SpiSck(1), UartCts(0), I2cSda(1), Pwm(7, 'A'), aux(UartTx(0)))),
    gpio_pin!(31, "GPIO31", GPIO, rp!(SpiMosi(1), UartRts(0), I2cScl(1), Pwm(7, 'B'), aux(UartRx(0)))),
    gpio_pin!(32, "GPIO32", GPIO, rp!(SpiMiso(0), UartTx(0), I2cSda(0), Pwm(8, 'A'))),
    gpio_pin!(33, "GPIO33", GPIO, rp!(SpiCs(0, 0), UartRx(0), I2cScl(0), Pwm(8, 'B'))),
    gpio_pin!(34, "GPIO34", GPIO, rp!(SpiSck(0), UartCts(0), I2cSda(1), Pwm(9, 'A'), aux(UartTx(0)))),
    gpio_pin!(35, "GPIO35", GPIO, rp!(SpiMosi(0), UartRts(0), I2cScl(1), Pwm(9, 'B'), aux(UartRx(0)))),
    gpio_pin!(36, "GPIO36", GPIO, rp!(SpiMiso(0), UartTx(1), I2cSda(0), Pwm(10, 'A'))),
    gpio_pin!(37, "GPIO37", GPIO, rp!(SpiCs(0, 0), UartRx(1), I2cScl(0), Pwm(10, 'B'))),
    gpio_pin!(38, "GPIO38", GPIO, rp!(SpiSck(0), UartCts(1), I2cSda(1), Pwm(11, 'A'), aux(UartTx(1)))),
    gpio_pin!(39, "GPIO39", GPIO, rp!(SpiMosi(0), UartRts(1), I2cScl(1), Pwm(11, 'B'), aux(UartRx(1)))),
    gpio_pin!(40, "GPIO40", ADC, rp!(SpiMiso(1), UartTx(1), I2cSda(0), Pwm(8, 'A'), adc(0))),
    gpio_pin!(41, "GPIO41", ADC, rp!(SpiCs(1, 0), UartRx(1), I2cScl(0), Pwm(8, 'B'), adc(1))),
    gpio_pin!(42, "GPIO42", ADC, rp!(SpiSck(1), UartCts(1), I2cSda(1), Pwm(9, 'A'), aux(UartTx(1)), adc(2))),
    gpio_pin!(43, "GPIO43", ADC, rp!(SpiMosi(1), UartRts(1), I2cScl(1), Pwm(9, 'B'), aux(UartRx(1)), adc(3))),
    gpio_pin!(44, "GPIO44", ADC, rp!(SpiMiso(1), UartTx(0), I2cSda(0), Pwm(10, 'A'), adc(4))),
    gpio_pin!(45, "GPIO45", ADC, rp!(SpiCs(1, 0), UartRx(0), I2cScl(0), Pwm(10, 'B'), adc(5))),
    gpio_pin!(46, "GPIO46", ADC, rp!(SpiSck(1), UartCts(0), I2cSda(1), Pwm(11, 'A'), aux(UartTx(0)), adc(6))),
    gpio_pin!(47, "GPIO47", ADC, rp!(SpiMosi(1), UartRts(0), I2cScl(1), Pwm(11, 'B'), aux(UartRx(0)), adc(7))),
];

pub static RP2350B: SiliconTarget = SiliconTarget {
    name: "rp2350b",
    family: "Raspberry Pi",
    pins: RP2350B_PINS,
};
