//! Code generation for firmware HAL and pin bindings.
//!
//! [`board_bindings`] builds one [`BoardBindings`] model from the netlist and
//! the silicon tables; `pins.rs` and `board.rs` ([`rust`]), `pins.h` ([`c`])
//! and the Zephyr overlay ([`zephyr`], experimental) render it.

mod adc;
mod buses;
pub mod c;
mod groups;
mod hal;
pub mod model;
mod naming;
pub mod pins;
mod render;
mod role;
pub mod rust;
mod select;
mod spi;
mod view;
pub mod zephyr;

#[cfg(test)]
mod tests;

pub(crate) use groups::{categorize_pins, field_names};
pub use model::{board_bindings, BoardBindings};
pub use pins::{extract_mcu_pins, generate_pins_rs, McuInfo, PinMapping};
pub(crate) use select::mcu_hint;
