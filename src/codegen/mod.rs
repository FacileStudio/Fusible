//! Code generation for firmware HAL and pin bindings.
//!
//! [`board_bindings`] builds one [`BoardBindings`] model from the netlist and
//! the silicon tables; `pins.rs` and `board.rs` ([`rust`]), `pins.h` ([`c`])
//! and the Zephyr overlay ([`zephyr`], experimental) render it.

mod adc;
mod bind;
mod buses;
pub mod c;
mod groups;
mod indicators;
pub mod model;
mod naming;
mod parts;
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

pub use bind::board_bindings;
pub(crate) use groups::{categorize_pins, field_names};
pub use model::BoardBindings;
pub use pins::{extract_mcu_pins, generate_pins_rs, McuInfo, PinMapping};
pub(crate) use select::mcu_hint;
