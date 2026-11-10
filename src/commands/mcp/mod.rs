//! Subcommand: `fusible mcp` — serve the project to AI agents over MCP on stdio.

mod config_snippets;
mod diag_json;
mod edit;
mod git;
mod guard;
pub(crate) mod report;
mod resources;
mod rpc;
mod session;
mod stdio;
mod tools;

pub(crate) use stdio::protocol_out;

use miette::Diagnostic;
use thiserror::Error;

use crate::cli::McpArgs;
use session::Session;

/// A server that could not start or keep its transport.
#[derive(Debug, Error, Diagnostic)]
pub enum McpError {
    #[error("cannot open the project: {0}")]
    #[diagnostic(code(fusible::mcp::project))]
    Project(String),

    #[error("transport failure: {0}")]
    #[diagnostic(code(fusible::mcp::io))]
    Io(#[from] std::io::Error),
}

/// Prints a client configuration snippet, or serves MCP until stdin closes.
pub fn run(args: McpArgs) -> Result<(), McpError> {
    if let Some(client) = args.print_config {
        print!("{}", config_snippets::snippet(client));
        return Ok(());
    }
    let session = Session::open(args.root).map_err(McpError::Project)?;
    stdio::serve(session)?;
    Ok(())
}
