//! RP2040 function tables of GPIO16 to GPIO29 (F1 SPI, F2 UART, F3 I2C,
//! F4 PWM, and the ADC inputs), from the datasheet's function select table (Table 279),
//! checked row by row against pico-sdk `src/rp2040/hardware_regs/.../io_bank0.h`.

use crate::silicon::model::{MuxFunction, PeripheralSignal};

pub(super) static GP16_FUNCS: &[MuxFunction] = &[
    mux!(PeripheralSignal::SpiMiso(0), 1, "F1"),
    mux!(PeripheralSignal::UartTx(0), 2, "F2"),
    mux!(PeripheralSignal::I2cSda(0), 3, "F3"),
    mux!(PeripheralSignal::Pwm(0, 'A'), 4, "F4"),
];

pub(super) static GP17_FUNCS: &[MuxFunction] = &[
    mux!(PeripheralSignal::SpiCs(0, 0), 1, "F1"),
    mux!(PeripheralSignal::UartRx(0), 2, "F2"),
    mux!(PeripheralSignal::I2cScl(0), 3, "F3"),
    mux!(PeripheralSignal::Pwm(0, 'B'), 4, "F4"),
];

pub(super) static GP18_FUNCS: &[MuxFunction] = &[
    mux!(PeripheralSignal::SpiSck(0), 1, "F1"),
    mux!(PeripheralSignal::UartCts(0), 2, "F2"),
    mux!(PeripheralSignal::I2cSda(1), 3, "F3"),
    mux!(PeripheralSignal::Pwm(1, 'A'), 4, "F4"),
];

pub(super) static GP19_FUNCS: &[MuxFunction] = &[
    mux!(PeripheralSignal::SpiMosi(0), 1, "F1"),
    mux!(PeripheralSignal::UartRts(0), 2, "F2"),
    mux!(PeripheralSignal::I2cScl(1), 3, "F3"),
    mux!(PeripheralSignal::Pwm(1, 'B'), 4, "F4"),
];

pub(super) static GP20_FUNCS: &[MuxFunction] = &[
    mux!(PeripheralSignal::SpiMiso(0), 1, "F1"),
    mux!(PeripheralSignal::UartTx(1), 2, "F2"),
    mux!(PeripheralSignal::I2cSda(0), 3, "F3"),
    mux!(PeripheralSignal::Pwm(2, 'A'), 4, "F4"),
];

pub(super) static GP21_FUNCS: &[MuxFunction] = &[
    mux!(PeripheralSignal::SpiCs(0, 0), 1, "F1"),
    mux!(PeripheralSignal::UartRx(1), 2, "F2"),
    mux!(PeripheralSignal::I2cScl(0), 3, "F3"),
    mux!(PeripheralSignal::Pwm(2, 'B'), 4, "F4"),
];

pub(super) static GP22_FUNCS: &[MuxFunction] = &[
    mux!(PeripheralSignal::SpiSck(0), 1, "F1"),
    mux!(PeripheralSignal::UartCts(1), 2, "F2"),
    mux!(PeripheralSignal::I2cSda(1), 3, "F3"),
    mux!(PeripheralSignal::Pwm(3, 'A'), 4, "F4"),
];

pub(super) static GP23_FUNCS: &[MuxFunction] = &[
    mux!(PeripheralSignal::SpiMosi(0), 1, "F1"),
    mux!(PeripheralSignal::UartRts(1), 2, "F2"),
    mux!(PeripheralSignal::I2cScl(1), 3, "F3"),
    mux!(PeripheralSignal::Pwm(3, 'B'), 4, "F4"),
];

pub(super) static GP24_FUNCS: &[MuxFunction] = &[
    mux!(PeripheralSignal::SpiMiso(1), 1, "F1"),
    mux!(PeripheralSignal::UartTx(1), 2, "F2"),
    mux!(PeripheralSignal::I2cSda(0), 3, "F3"),
    mux!(PeripheralSignal::Pwm(4, 'A'), 4, "F4"),
];

pub(super) static GP25_FUNCS: &[MuxFunction] = &[
    mux!(PeripheralSignal::SpiCs(1, 0), 1, "F1"),
    mux!(PeripheralSignal::UartRx(1), 2, "F2"),
    mux!(PeripheralSignal::I2cScl(0), 3, "F3"),
    mux!(PeripheralSignal::Pwm(4, 'B'), 4, "F4"),
];

pub(super) static GP26_FUNCS: &[MuxFunction] = &[
    mux!(PeripheralSignal::Adc(0), 0, "ADC"),
    mux!(PeripheralSignal::SpiSck(1), 1, "F1"),
    mux!(PeripheralSignal::UartCts(1), 2, "F2"),
    mux!(PeripheralSignal::I2cSda(1), 3, "F3"),
    mux!(PeripheralSignal::Pwm(5, 'A'), 4, "F4"),
];

pub(super) static GP27_FUNCS: &[MuxFunction] = &[
    mux!(PeripheralSignal::Adc(1), 0, "ADC"),
    mux!(PeripheralSignal::SpiMosi(1), 1, "F1"),
    mux!(PeripheralSignal::UartRts(1), 2, "F2"),
    mux!(PeripheralSignal::I2cScl(1), 3, "F3"),
    mux!(PeripheralSignal::Pwm(5, 'B'), 4, "F4"),
];

pub(super) static GP28_FUNCS: &[MuxFunction] = &[
    mux!(PeripheralSignal::Adc(2), 0, "ADC"),
    mux!(PeripheralSignal::SpiMiso(1), 1, "F1"),
    mux!(PeripheralSignal::UartTx(0), 2, "F2"),
    mux!(PeripheralSignal::I2cSda(0), 3, "F3"),
    mux!(PeripheralSignal::Pwm(6, 'A'), 4, "F4"),
];

pub(super) static GP29_FUNCS: &[MuxFunction] = &[
    mux!(PeripheralSignal::Adc(3), 0, "ADC"),
    mux!(PeripheralSignal::SpiCs(1, 0), 1, "F1"),
    mux!(PeripheralSignal::UartRx(0), 2, "F2"),
    mux!(PeripheralSignal::I2cScl(0), 3, "F3"),
    mux!(PeripheralSignal::Pwm(6, 'B'), 4, "F4"),
];
