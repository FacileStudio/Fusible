//! Subcommand: `fusible bringup [BOARD]`: write the bench sheet `<out_dir>/bringup.md`.
//!
//! The sheet lists the rails in power order with their expected voltage and the point to
//! probe, the MCU strap pins and how the board sets them, and the pinout of every
//! connector reaching an MCU debug or programming pin. It reads `nets:`, the pin
//! attributes and the netlist. It documents the board and never blows the fuse.

mod debug;
mod order;
mod rails;
mod render;
mod straps;
mod view;

use std::path::PathBuf;

use miette::Diagnostic;
use thiserror::Error;

use crate::codegen::extract_mcu_pins;
use crate::commands::check::{load_circuit_source, CheckError};
use crate::commands::target::{resolve_board, BoardTarget};
use crate::netlist::Netlist;
use crate::parser::{self, Circuit};
use crate::project::Project;
use crate::silicon::find_target;

pub use render::{markdown, Sheet};

/// Why the sheet was not written.
#[derive(Debug, Error, Diagnostic)]
pub enum BringupError {
    #[error(transparent)]
    #[diagnostic(transparent)]
    Check(#[from] CheckError),

    #[error("cannot write \"{path}\"")]
    #[diagnostic(code(fusible::bringup::write))]
    Write {
        path: String,
        #[source]
        source: std::io::Error,
    },
}

/// Writes the sheet of `target` (or the only board) to `output`, by default
/// `<out_dir>/bringup.md`, and returns its path.
pub fn run(
    project: &Project,
    target: Option<PathBuf>,
    output: Option<PathBuf>,
) -> Result<PathBuf, BringupError> {
    let board = resolve_board(project, target).map_err(CheckError::from)?;
    let (json_path, source) = load_circuit_source(&board.entry)?;
    let circuit = parser::parse(&source, &json_path).map_err(CheckError::from)?;
    let netlist = Netlist::from_circuit(&circuit);
    let text = markdown(&sheet(project, &board, &circuit, &netlist));
    let path = output.unwrap_or_else(|| board.out_dir.join("bringup.md"));
    let write_err = |source| BringupError::Write {
        path: path.display().to_string(),
        source,
    };
    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent).map_err(write_err)?;
    }
    std::fs::write(&path, text).map_err(write_err)?;
    println!("📋 Wrote the bring-up sheet -> {}", path.display());
    Ok(path)
}

/// The sheet of one parsed board.
pub fn sheet(
    project: &Project,
    board: &BoardTarget,
    circuit: &Circuit,
    netlist: &Netlist,
) -> Sheet {
    let mcu = extract_mcu_pins(circuit, netlist, board.mcu.as_deref());
    let view = view::View {
        circuit,
        netlist,
        mcu_id: mcu.as_ref().map(|m| m.component_id.clone()),
    };
    let straps = mcu
        .as_ref()
        .and_then(|m| Some(straps::straps(&view, m, find_target(&m.chip)?)))
        .unwrap_or_default();
    Sheet {
        board: board.title.clone(),
        rails: rails::rails(&view, &project.design),
        connectors: debug::connectors(&view),
        mcu: mcu.map(|m| m.chip),
        straps,
    }
}
