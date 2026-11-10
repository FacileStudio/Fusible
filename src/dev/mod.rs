mod api;
mod assembly;
mod assets;
mod bind;
mod bodies;
mod bom;
mod bom_read;
mod desktop;
mod firmware;
mod fix_api;
mod git;
mod guard;
pub mod hub;
mod kicad;
mod limiter;
mod pair;
pub mod pipeline;
mod project;
mod refresh;
mod routes;
mod rules_api;
mod run;
mod serve;
pub mod server;
mod shared;
mod stamps;
mod state;
pub mod watcher;

#[cfg(test)]
mod api_tests;
#[cfg(test)]
mod bind_tests;
#[cfg(test)]
mod bom_tests;
#[cfg(test)]
mod desktop_tests;
#[cfg(test)]
mod firmware_tests;
#[cfg(test)]
mod guard_tests;
#[cfg(test)]
mod hub_tests;
#[cfg(test)]
mod limiter_tests;
#[cfg(test)]
mod panel_scripts_tests;
#[cfg(test)]
mod rules_api_tests;
#[cfg(test)]
mod stamps_tests;
#[cfg(test)]
mod state_tests;
#[cfg(test)]
mod studio_tests;
#[cfg(test)]
mod tests;

pub use server::run_dev_server;
