//! Subcommand: `fusible flash [FIRMWARE]`: check the board, then run the flash command.
//!
//! Fusible shells out and links no flash tool. Without `flash_cmd`, the framework picks one:
//! Embassy and cortex-m run `probe-rs run --chip <mcu>` as the cargo runner, esp-hal and esp-idf run
//! `espflash flash` the same way, esp-idf-c runs `idf.py flash`, Zephyr runs `west flash`.

use std::path::PathBuf;

use miette::Diagnostic;
use thiserror::Error;

use crate::commands::build::{execute_shell, missing_tool};
use crate::commands::check::{self, CheckError};
use crate::commands::target::resolve_board;
use crate::config::{FirmwareConfig, Framework};
use crate::project::Project;

mod tool;

/// Why `fusible flash` did not flash.
#[derive(Debug, Error, Diagnostic)]
pub enum FlashError {
    #[error(transparent)]
    #[diagnostic(transparent)]
    Check(#[from] CheckError),

    #[error("no firmware named `{0}` in fusible.yml")]
    #[diagnostic(
        code(fusible::flash::unknown),
        help("run `fusible doctor` to list the firmware")
    )]
    Unknown(String),

    #[error("this project declares {} firmware: name the one to flash", .0.len())]
    #[diagnostic(code(fusible::flash::choose), help("firmware: {}", .0.join(", ")))]
    Choose(Vec<String>),

    #[error("firmware `{0}` has no flash command")]
    #[diagnostic(
        code(fusible::flash::no_command),
        help("set `flash_cmd`, or a `framework` and the board `mcu` (letters, digits, - and _)")
    )]
    NoCommand(String),

    #[error("flash tool not found: {tool}")]
    #[diagnostic(code(fusible::flash::tool_missing), help("{hint}"))]
    ToolMissing { tool: String, hint: String },

    #[error("flash command failed: {0}")]
    #[diagnostic(
        code(fusible::flash::failed),
        help("check the probe or the USB cable and the chip name")
    )]
    Failed(String),

    #[error("cannot run the flash command: {0}")]
    #[diagnostic(code(fusible::flash::spawn))]
    Spawn(#[from] std::io::Error),
}

/// Flashes `target` (or the only firmware) once its board keeps the fuse intact.
pub fn run(project: &Project, target: Option<String>) -> Result<(), FlashError> {
    let (name, fw) = select(project, target)?;
    let board =
        resolve_board(project, fw.board.clone().map(PathBuf::from)).map_err(CheckError::from)?;
    let cmd =
        command(fw, board.mcu.as_deref()).ok_or_else(|| FlashError::NoCommand(name.clone()))?;
    check::run_boards(project, std::slice::from_ref(&board))?;
    let path = std::env::var_os("PATH").unwrap_or_default();
    if let Some(tool) = tool::missing(&cmd, fw.path.as_deref(), &path) {
        return Err(FlashError::missing(tool));
    }
    println!("⚡ Flashing firmware {name}: \"{cmd}\"...");
    let status = execute_shell(&cmd, fw.path.as_deref())?;
    if !status.success() {
        return Err(match missing_tool(&cmd, status.code()) {
            Some(tool) => FlashError::missing(tool),
            None => FlashError::Failed(cmd),
        });
    }
    println!("✅ Flashed firmware {name}");
    Ok(())
}

impl FlashError {
    fn missing(tool: String) -> Self {
        let hint = tool::install_hint(&tool);
        Self::ToolMissing { tool, hint }
    }
}

fn select(
    project: &Project,
    target: Option<String>,
) -> Result<(&String, &FirmwareConfig), FlashError> {
    let firmware = &project.config.firmware;
    match target {
        Some(name) => firmware
            .get_key_value(&name)
            .ok_or(FlashError::Unknown(name)),
        None if firmware.len() == 1 => firmware.iter().next().ok_or(FlashError::Choose(Vec::new())),
        None => Err(FlashError::Choose(firmware.keys().cloned().collect())),
    }
}

/// `flash_cmd`, else the framework default. A chip name outside `[A-Za-z0-9_-]` gives none.
pub fn command(fw: &FirmwareConfig, chip: Option<&str>) -> Option<String> {
    if let Some(cmd) = &fw.flash_cmd {
        return Some(cmd.clone());
    }
    let runner = |tool: String| {
        format!("cargo run --release --config \"target.'cfg(all())'.runner='{tool}'\"")
    };
    let chip = chip.filter(|c| {
        !c.is_empty()
            && c.chars()
                .all(|c| c.is_ascii_alphanumeric() || "-_".contains(c))
    });
    match fw.framework_kind()? {
        Framework::Embassy | Framework::CortexM => {
            Some(runner(format!("probe-rs run --chip {}", chip?)))
        }
        Framework::EspHal | Framework::EspIdf => Some(runner("espflash flash".to_string())),
        Framework::EspIdfC => Some("idf.py flash".to_string()),
        Framework::Zephyr => Some("west flash".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::command;
    use crate::config::FirmwareConfig;

    fn fw(framework: &str) -> FirmwareConfig {
        FirmwareConfig {
            framework: Some(framework.to_string()),
            ..Default::default()
        }
    }

    #[test]
    fn the_framework_picks_the_default_flash_command() {
        let embassy = command(&fw("embassy"), Some("STM32G431CBUx")).unwrap();
        assert!(
            embassy.contains("runner='probe-rs run --chip STM32G431CBUx'"),
            "{embassy}"
        );
        assert!(command(&fw("embassy"), Some("x; rm -rf /")).is_none());
        assert!(command(&fw("esp-idf"), None)
            .unwrap()
            .contains("espflash flash"));
        assert!(command(&fw("esp-hal"), None)
            .unwrap()
            .contains("espflash flash"));
        assert_eq!(command(&fw("zephyr"), None).as_deref(), Some("west flash"));
        assert_eq!(
            command(&fw("esp-idf-c"), None).as_deref(),
            Some("idf.py flash")
        );
        assert!(command(&fw("arduino"), None).is_none());
    }
}
