//! CLI subcommands implementation.

pub mod add;
pub mod assembly;
pub mod bindings;
pub mod body;
pub mod bom;
pub mod build;
pub mod check;
pub mod dev;
pub mod diff;
pub mod doctor;
pub mod export;
pub mod harness;
pub mod init;
pub mod install;
pub mod links;
pub mod mcp;
pub mod migrate;
pub mod opus;
pub mod release;
pub mod route;
pub mod search;
pub mod share;
pub mod source;
pub mod sync;
pub mod target;

#[cfg(test)]
mod links_tests;
#[cfg(test)]
mod tests;
