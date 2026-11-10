//! Runs fuse-gated subcommands and maps their results onto the exit-code contract.

mod body;
mod json;
mod misc;
mod release;

pub use body::body;
use json::check_json;
pub use misc::{init, migrate, share};
pub use release::release;

use std::path::PathBuf;
use std::process::ExitCode;

use crate::cli::{AssemblyArgs, CheckArgs, ExportArgs, RouteArgs, StudioArgs, SyncArgs};
use crate::commands;
use crate::commands::check::CheckError;
use crate::project::Project;
use crate::router::RouteOptions;
use crate::{fail, tool_exit, EXIT_TOOL_ERROR, EXIT_VIOLATIONS};

/// Runs the dev server and forwards its exit code.
pub fn dev(project: &Project, options: StudioArgs) -> ExitCode {
    match commands::dev::run(project, options) {
        Ok(code) => ExitCode::from(u8::try_from(code).unwrap_or(EXIT_TOOL_ERROR)),
        Err(err) => fail(err, EXIT_TOOL_ERROR),
    }
}

/// Runs the electrical linter; with `--opus`, a blown fuse also opens Opus tasks.
pub fn check(project: &Project, args: CheckArgs) -> ExitCode {
    let CheckArgs {
        file,
        mcu,
        pins_rs,
        json,
        opus,
    } = args;
    if json {
        return check_json(project, file, mcu);
    }
    match commands::check::run(project, file.clone(), mcu, pins_rs) {
        Ok(()) => ExitCode::SUCCESS,
        Err(CheckError::FuseBlown(blown)) => {
            if opus.opus {
                commands::opus::report_check(project, file, &opus);
            }
            fail(blown, EXIT_VIOLATIONS)
        }
        Err(err) => fail(err, EXIT_TOOL_ERROR),
    }
}

/// Runs the full build pipeline.
pub fn build(project: &Project, file: Option<PathBuf>) -> ExitCode {
    match commands::build::run(project, file) {
        Ok(()) => ExitCode::SUCCESS,
        Err(commands::build::BuildError::Check(CheckError::FuseBlown(blown))) => {
            fail(blown, EXIT_VIOLATIONS)
        }
        Err(err) => fail(err, EXIT_TOOL_ERROR),
    }
}

/// Runs a fuse-gated export.
pub fn export(project: &Project, args: ExportArgs) -> ExitCode {
    match commands::export::run(project, args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(commands::export::ExportError::Check(CheckError::FuseBlown(blown))) => {
            fail(blown, EXIT_VIOLATIONS)
        }
        Err(err) => fail(err, EXIT_TOOL_ERROR),
    }
}

/// Builds the fuse-gated assembly model.
pub fn assembly(project: &Project, args: AssemblyArgs) -> ExitCode {
    match commands::assembly::run(project, args.format, args.output, args.skip_lint) {
        Ok(()) => ExitCode::SUCCESS,
        Err(commands::assembly::AssemblyError::Check(CheckError::FuseBlown(blown))) => {
            fail(blown, EXIT_VIOLATIONS)
        }
        Err(err) => fail(err, EXIT_TOOL_ERROR),
    }
}

/// Runs the autorouter; unrouted nets blow the fuse.
pub fn route(project: &Project, args: RouteArgs) -> ExitCode {
    let options = RouteOptions {
        clearance: args.clearance,
        trace_width: args.trace_width,
        layers: args.layers,
        dry_run: args.dry_run,
        ..Default::default()
    };
    match commands::route::run(project, args.file, args.output, options) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err @ commands::route::RouteCommandError::UnroutedNets { .. }) => {
            fail(err, EXIT_VIOLATIONS)
        }
        Err(err) => fail(err, EXIT_TOOL_ERROR),
    }
}

/// Compares KiCad placement with tscircuit; drift blows the fuse.
pub fn diff(project: &Project, kicad_pcb: PathBuf, file: Option<PathBuf>) -> ExitCode {
    match commands::diff::run(project, kicad_pcb, file) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err @ commands::diff::DiffCommandError::DriftDetected) => fail(err, EXIT_VIOLATIONS),
        Err(err) => fail(err, EXIT_TOOL_ERROR),
    }
}

/// Syncs KiCad layout edits back into manual-edits.json.
pub fn sync(project: &Project, args: SyncArgs) -> ExitCode {
    tool_exit(commands::sync::run(project, args))
}
