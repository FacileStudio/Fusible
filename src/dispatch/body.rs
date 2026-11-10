//! Dispatch of `fusible body`: build, eject, and kit.

use std::process::ExitCode;

use crate::cli::{BodyAction, BodyArgs};
use crate::commands;
use crate::commands::body::BodyError;
use crate::commands::check::CheckError;
use crate::config::EnclosureConfig;
use crate::project::Project;
use crate::{fail, EXIT_TOOL_ERROR, EXIT_VIOLATIONS};

/// Maps a body result onto the exit-code contract.
fn body_exit(result: Result<(), BodyError>) -> ExitCode {
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(BodyError::Check(CheckError::FuseBlown(blown))) => fail(blown, EXIT_VIOLATIONS),
        Err(err) => fail(err, EXIT_TOOL_ERROR),
    }
}

/// Builds the project bodies, or ejects one or writes a kit; CLI flags override enclosure settings.
pub fn body(project: &Project, mut args: BodyArgs) -> ExitCode {
    match args.action.take() {
        Some(BodyAction::Eject(a)) => {
            return body_exit(commands::body::eject(
                project,
                a.name,
                a.output,
                a.skip_lint,
            ));
        }
        Some(BodyAction::Kit(a)) => {
            return body_exit(commands::body::kit(project, a.board, a.output, a.skip_lint));
        }
        Some(BodyAction::New(a)) => {
            return body_exit(commands::body::new(
                project, a.name, a.board, a.material, a.process,
            ))
        }
        None => {}
    }
    let (target, skip_lint) = (args.target.clone(), args.skip_lint);
    body_exit(commands::body::run(
        project,
        target,
        overrides(args),
        skip_lint,
    ))
}

fn overrides(args: BodyArgs) -> EnclosureConfig {
    EnclosureConfig {
        format: args.format,
        out_dir: args.output,
        wall_thickness: args.wall_thickness,
        height: args.height,
        cutouts: args.no_cutouts.then_some(false),
        ventilation: args.ventilation,
        mounting: args.no_mounting.then_some(false),
    }
}
