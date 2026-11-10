//! The project a command runs in: its root, its manifest, and the design rules it declares.
//!
//! `main` loads one [`Project`] and hands it to every command. Long-running hosts (the
//! studio, the MCP server) call [`Project::reload`] or [`Project::current`] at the start of
//! each request, because the manifest can change on disk while they run.

mod context;
#[cfg(test)]
mod tests;

pub use context::Project;
