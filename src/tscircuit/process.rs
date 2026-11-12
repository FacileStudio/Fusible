//! Process execution helpers shared by every runner command.

use std::process::{Child, Command, ExitStatus, Output, Stdio};

use super::deadline::{output_within, timeout, wait_with_timeout};
use crate::tscircuit::RunnerError;

/// Formats a command for diagnostic display.
pub fn format_command(cmd: &Command) -> String {
    let program = cmd.get_program().to_string_lossy();
    let args: Vec<_> = cmd.get_args().map(|a| a.to_string_lossy()).collect();
    if args.is_empty() {
        program.into_owned()
    } else {
        format!("{program} {}", args.join(" "))
    }
}

/// Extracts error details from a command's stderr and stdout.
pub fn extract_output_details(out: &Output) -> String {
    let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
    if !stderr.is_empty() {
        return stderr;
    }
    let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if !stdout.is_empty() {
        return stdout;
    }
    format!("command failed: {}", out.status)
}

/// Connects the command's stdin, stdout and stderr to the terminal.
pub fn inherit_stdio(cmd: &mut Command) {
    cmd.stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
}

/// Runs `cmd` attached to the terminal; a non-zero exit becomes `CommandFailed`.
pub fn run_inherited(mut cmd: Command, stderr: &str, help: &str) -> Result<(), RunnerError> {
    inherit_stdio(&mut cmd);
    let status = status(&mut cmd)?;

    if !status.success() {
        return Err(RunnerError::CommandFailed {
            command: format_command(&cmd),
            code: status.code(),
            stderr: stderr.into(),
            help: help.into(),
        });
    }
    Ok(())
}

/// Runs `cmd` with the stdio it was given, killing it after `FUSIBLE_BUILD_TIMEOUT_SECS`.
pub fn status(cmd: &mut Command) -> Result<ExitStatus, RunnerError> {
    let mut child = spawn(cmd)?;
    wait_with_timeout(&mut child, &format_command(cmd), timeout())
}

/// Runs `cmd` with no stdin and piped output, killing it after `FUSIBLE_BUILD_TIMEOUT_SECS`.
pub fn output(cmd: &mut Command) -> Result<Output, RunnerError> {
    output_within(cmd, timeout())
}

/// Starts `cmd`, reporting a spawn failure as `RunnerError::Io`.
pub(super) fn spawn(cmd: &mut Command) -> Result<Child, RunnerError> {
    cmd.spawn().map_err(|source| RunnerError::Io {
        command: format_command(cmd),
        source,
    })
}
