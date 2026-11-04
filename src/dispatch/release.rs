//! Dispatch of `fusible release`.

use std::process::ExitCode;

use crate::cli::ReleaseArgs;
use crate::commands;
use crate::commands::check::CheckError;
use crate::commands::release::ReleaseError;
use crate::{fail, EXIT_TOOL_ERROR, EXIT_VIOLATIONS};

/// Runs the fuse-gated release; a blown fuse or a warning exits 1, any other failure exits 2.
pub fn release(args: ReleaseArgs) -> ExitCode {
    match commands::release::run(args.target, &args.out, args.allow_warnings) {
        Ok(()) => ExitCode::SUCCESS,
        Err(ReleaseError::Check(CheckError::FuseBlown(blown))) => fail(blown, EXIT_VIOLATIONS),
        Err(err @ ReleaseError::Warnings { .. }) => fail(err, EXIT_VIOLATIONS),
        Err(err) => fail(err, EXIT_TOOL_ERROR),
    }
}
