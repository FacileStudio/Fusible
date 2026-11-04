//! Dispatch of `fusible init` and `fusible share`.

use std::process::ExitCode;

use crate::cli::{InitArgs, ShareArgs};
use crate::commands;
use crate::{fail, tool_exit, EXIT_TOOL_ERROR, EXIT_VIOLATIONS};

/// Initializes a project.
pub fn init(args: InitArgs) -> ExitCode {
    let InitArgs {
        path,
        name,
        mcu,
        currency,
        no_install,
    } = args;
    tool_exit(commands::init::run(path, name, mcu, currency, no_install))
}

/// Shares the fuse-gated export; a blown fuse exits 1.
pub fn share(args: ShareArgs) -> ExitCode {
    match commands::share::run(args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) if err.is_fuse_blown() => fail(err, EXIT_VIOLATIONS),
        Err(err) => fail(err, EXIT_TOOL_ERROR),
    }
}
