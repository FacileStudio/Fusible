//! Firmware bindings (`pins.rs`, `board.rs`, `pins.h`, Zephyr overlay), written by
//! `fusible build` and `check --write-bindings`.

use std::path::{Path, PathBuf};

use crate::check::{CheckError, ReadError};
use crate::codegen::{
    board_bindings, c, extract_mcu_pins, generate_pins_rs, mcu_hint, rust, zephyr, BoardBindings,
};
use crate::commands::source::load_circuit_source;
use crate::commands::target::BoardTarget;
use crate::config::OutputKind;
use crate::netlist::Netlist;
use crate::parser;
use crate::project::Project;

/// One file to generate.
struct Job {
    kind: OutputKind,
    path: PathBuf,
    board_rs: bool,
    zephyr: zephyr::Overrides,
}

/// Writes the bindings of every board into the `outputs:` of the firmware bound to it,
/// or only `pins.rs` into `path` when given. Call only once the fuse is intact.
pub fn write(
    project: &Project,
    boards: &[BoardTarget],
    path: Option<PathBuf>,
) -> Result<(), CheckError> {
    for board in boards {
        let jobs = jobs(project, board, path.clone());
        if !jobs.is_empty() {
            write_board(board, &jobs)?;
        }
    }
    Ok(())
}

fn jobs(project: &Project, board: &BoardTarget, path: Option<PathBuf>) -> Vec<Job> {
    if let Some(path) = path {
        return vec![Job {
            kind: OutputKind::Rust,
            path,
            board_rs: false,
            zephyr: zephyr::Overrides::new(),
        }];
    }
    let firmware = board
        .firmware
        .iter()
        .filter_map(|n| project.config.firmware.get(n));
    firmware
        .flat_map(|fw| {
            let board_rs = fw.wants_board_rs();
            fw.output_paths().into_iter().map(move |(kind, path)| Job {
                board_rs: board_rs && kind == OutputKind::Rust,
                kind,
                path,
                zephyr: fw.zephyr.clone(),
            })
        })
        .collect()
}

fn write_board(board: &BoardTarget, jobs: &[Job]) -> Result<(), CheckError> {
    let (json_path, source) = load_circuit_source(&board.entry)?;
    let circuit = parser::parse(&source, &json_path)?;
    let netlist = Netlist::from_circuit(&circuit);
    let mcu = board.mcu.as_deref();
    let Some(info) = extract_mcu_pins(&circuit, &netlist, mcu) else {
        if let Some(hint) = mcu_hint(&circuit, mcu) {
            eprintln!("{hint}");
        }
        return Ok(());
    };
    let bindings = board_bindings(&board.title, info, &circuit, &netlist);
    for (path, code) in jobs.iter().flat_map(|job| render(job, &bindings)) {
        save(&path, &code)?;
    }
    Ok(())
}

fn render(job: &Job, b: &BoardBindings) -> Vec<(PathBuf, String)> {
    let mut files = vec![match job.kind {
        OutputKind::Rust => (job.path.clone(), generate_pins_rs(&b.mcu, &b.board)),
        OutputKind::C => (job.path.clone(), c::pins_h(b)),
        OutputKind::Zephyr => (job.path.clone(), overlay(job, b)),
    }];
    if job.board_rs {
        let board_rs = rust::board_rs(b);
        files.extend(board_rs.map(|code| (job.path.with_file_name("board.rs"), code)));
    }
    files
}

/// The overlay text; each part it could not describe is a warning naming the file.
fn overlay(job: &Job, b: &BoardBindings) -> String {
    let overlay = zephyr::overlay(b, &job.zephyr);
    for todo in &overlay.todo {
        eprintln!("warning: {}: {todo}", job.path.display());
    }
    overlay.text
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
