//! The live boards: their slots, the check pipeline, file watching, rebuilds and the event hub.

pub mod hub;
pub mod pipeline;
pub(super) mod refresh;
pub(super) mod shared;
pub(super) mod stamps;
pub(super) mod state;
pub mod watcher;

#[cfg(test)]
mod hub_tests;
#[cfg(test)]
mod stamps_tests;
#[cfg(test)]
mod state_tests;
