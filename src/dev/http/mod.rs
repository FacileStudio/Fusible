//! The studio HTTP server: binding, admission, routing, static assets and the desktop hooks.

pub(super) mod assets;
pub(super) mod bind;
pub(super) mod desktop;
pub(super) mod guard;
pub(super) mod limiter;
pub(super) mod routes;
pub(super) mod serve;
pub mod server;

#[cfg(test)]
mod bind_tests;
#[cfg(test)]
mod desktop_tests;
#[cfg(test)]
mod guard_tests;
#[cfg(test)]
mod limiter_tests;
#[cfg(test)]
mod panel_scripts_tests;
