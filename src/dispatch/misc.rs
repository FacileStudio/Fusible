//! Dispatch of `fusible init`, `fusible migrate` and `fusible share`.

use std::process::ExitCode;

use crate::cli::{InitArgs, ShareArgs};
use crate::commands;
use crate::commands::migrate::Migration;
use crate::project::Project;
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

/// Migrates the manifest; with `--check`, a manifest that needs a rewrite exits 1.
pub fn migrate(check: bool) -> ExitCode {
    match commands::migrate::run(check) {
        Ok(Migration::Needed) => ExitCode::from(EXIT_VIOLATIONS),
        Ok(Migration::Current | Migration::Written) => ExitCode::SUCCESS,
        Err(err) => fail(err, EXIT_TOOL_ERROR),
    }
}

/// Shares the fuse-gated export; a blown fuse exits 1.
pub fn share(project: &Project, args: ShareArgs) -> ExitCode {
    match commands::share::run(project, args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) if err.is_fuse_blown() => fail(err, EXIT_VIOLATIONS),
        Err(err) => fail(err, EXIT_TOOL_ERROR),
    }
}
