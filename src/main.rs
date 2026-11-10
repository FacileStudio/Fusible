//! Fusible — electronic linter and tscircuit project manager.
//!
//! Exit codes are part of the contract (they let CI distinguish what failed):
//! * `0` — fuse intact, command succeeded;
//! * `1` — electrical violations: the fuse blew, generation is blocked;
//! * `2` — tool error (unreadable/malformed input, runner failure, usage error).

pub mod bom;
pub mod check;
pub mod cli;
pub mod codegen;
pub mod commands;
pub mod config;
pub mod dev;
mod dispatch;
pub mod gerber;
pub mod kicad;
pub mod linter;
pub mod netlist;
pub mod parser;
pub mod project;
pub mod router;
pub mod silicon;
pub mod svg;
#[cfg(test)]
mod test_shell;
pub mod tscircuit;

use std::process::ExitCode;

use clap::Parser;
use miette::Diagnostic;

use crate::cli::{CheckArgs, Cli, Commands, OpusArgs};
use crate::project::Project;

/// Exit code: electrical violations (the fuse blew).
pub const EXIT_VIOLATIONS: u8 = 1;
/// Exit code: the tool itself could not do its job (aligns with clap's 2).
pub const EXIT_TOOL_ERROR: u8 = 2;

fn main() -> ExitCode {
    let cli = Cli::parse();
    let project = match load_project(cli.command.as_ref()) {
        Ok(project) => project,
        Err(err) => return fail(err, EXIT_TOOL_ERROR),
    };
    match cli.command {
        Some(command) => run_command(command, &project),
        None => dispatch::check(
            &project,
            CheckArgs {
                file: cli.input,
                mcu: cli.mcu,
                write_bindings: false,
                pins_rs: None,
                json: false,
                opus: OpusArgs::default(),
            },
        ),
    }
}

/// Loads the project of the working directory once, for every command.
///
/// `init`, `doctor`, `mcp` and `migrate` run without a valid manifest, so a manifest error fails only
/// the other commands.
fn load_project(command: Option<&Commands>) -> Result<Project, config::ConfigError> {
    let tolerant = matches!(
        command,
        Some(
            Commands::Init(_)
                | Commands::Doctor { .. }
                | Commands::Mcp(_)
                | Commands::Migrate { .. }
        )
    );
    match tolerant {
        true => Ok(Project::current()),
        false => Project::load(std::path::Path::new(".")),
    }
}

/// Dispatches one subcommand and returns its exit code.
fn run_command(command: Commands, project: &Project) -> ExitCode {
    match command {
        Commands::Studio(args) => dispatch::dev(project, args),
        Commands::Check(args) => dispatch::check(project, args),
        Commands::Build { file } => dispatch::build(project, file),
        Commands::Export(args) => dispatch::export(project, args),
        Commands::Release(args) => dispatch::release(project, args),
        Commands::Body(args) => dispatch::body(project, args),
        Commands::Assembly(args) => dispatch::assembly(project, args),
        Commands::Bom(args) => tool_exit(commands::bom::run(project, args)),
        Commands::Mcp(args) => tool_exit(commands::mcp::run(args)),
        Commands::Share(args) => dispatch::share(project, args),
        Commands::Route(args) => dispatch::route(project, args),
        Commands::Diff { kicad_pcb, file } => dispatch::diff(project, kicad_pcb, file),
        Commands::Sync(args) => dispatch::sync(project, args),
        Commands::Doctor { install, json } => tool_exit(commands::doctor::run(install, json)),
        Commands::Init(args) => dispatch::init(args),
        Commands::Migrate { check } => dispatch::migrate(check),
        Commands::Add { packages } => tool_exit(commands::add::run(project, &packages)),
        Commands::Install { dir } => tool_exit(commands::install::run(project, dir)),
        Commands::Search { query } => tool_exit(commands::search::run(&query)),
    }
}

/// Maps a command result where every error is a tool error.
fn tool_exit<E: Diagnostic + Send + Sync + 'static>(result: Result<(), E>) -> ExitCode {
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => fail(err, EXIT_TOOL_ERROR),
    }
}

/// Renders an error through miette's fancy handler on stderr, then returns
/// the mapped exit code.
fn fail(error: impl Diagnostic + Send + Sync + 'static, code: u8) -> ExitCode {
    eprintln!("Error: {:?}", miette::Report::new(error));
    ExitCode::from(code)
}
