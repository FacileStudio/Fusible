//! Authored bodies: a file made in any CAD tool, optionally produced by a `build:` command.

use std::path::Path;

use super::BodyError;
use crate::commands::build::execute_shell;
use crate::config::BodyConfig;

/// Runs the `build:` command of an authored body from `root`, then checks that its `source` exists.
pub(crate) fn build_authored(name: &str, body: &BodyConfig, root: &Path) -> Result<(), BodyError> {
    let path = body.source.clone().unwrap_or_default();
    if let Some(command) = &body.build {
        println!("🛠️  Building body `{name}`: \"{command}\"...");
        let failed = || BodyError::BuildFailed {
            name: name.to_string(),
            command: command.clone(),
        };
        let status = execute_shell(command, Some(root)).map_err(|_| failed())?;
        status.success().then_some(()).ok_or_else(failed)?;
    }
    if !path.is_file() {
        return Err(BodyError::MissingSource {
            name: name.to_string(),
            path,
        });
    }
    println!("✅ Body {name} (authored) -> {}", path.display());
    Ok(())
}
