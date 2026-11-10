//! Firmware pin bindings (`pins.rs`), written by `fusible build` and `check --write-bindings`.

use std::path::{Path, PathBuf};

use crate::check::{CheckError, ReadError};
use crate::codegen::{extract_mcu_pins, generate_pins_rs};
use crate::commands::source::load_circuit_source;
use crate::commands::target::BoardTarget;
use crate::netlist::Netlist;
use crate::parser;

/// Writes the pin bindings of every board into its firmware's `pins_rs` files, or into
/// `path` when given. Call only once the fuse is intact.
pub fn write(boards: &[BoardTarget], path: Option<PathBuf>) -> Result<(), CheckError> {
    for board in boards {
        let targets = path
            .clone()
            .map_or_else(|| board.pins_rs.clone(), |p| vec![p]);
        if !targets.is_empty() {
            write_board(board, &targets)?;
        }
    }
    Ok(())
}

fn write_board(board: &BoardTarget, targets: &[PathBuf]) -> Result<(), CheckError> {
    let (json_path, source) = load_circuit_source(&board.entry)?;
    let circuit = parser::parse(&source, &json_path)?;
    let netlist = Netlist::from_circuit(&circuit);
    let mcu = board.mcu.as_deref();
    let Some(info) = extract_mcu_pins(&circuit, &netlist, mcu) else {
        return Ok(());
    };
    let code = generate_pins_rs(&info, &board.title);
    targets.iter().try_for_each(|path| save(path, &code))
}

fn save(path: &Path, code: &str) -> Result<(), CheckError> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).ok();
    }
    std::fs::write(path, code).map_err(|source| ReadError {
        path: path.display().to_string(),
        source,
    })?;
    println!("🦀 Generated firmware pin bindings -> {}", path.display());
    Ok(())
}
