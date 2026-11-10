//! Tool `render`: SVG of a board, plus a PNG when ImageMagick is installed.

use std::io::Write;
use std::process::{Command, Stdio};

use serde_json::{json, Value};

use super::args::{choice, object, required_str, string};
use super::outcome::Outcome;
use crate::commands::check::load_circuit_source;
use crate::commands::mcp::report;
use crate::commands::mcp::session::Session;
use crate::netlist::Netlist;
use crate::project::Project;
use crate::{parser, svg};

pub const DESCRIPTION: &str = "Render the PCB layout or the schematic of a board as SVG. \
Adds a PNG image when ImageMagick (magick) is on PATH.";

const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// Input schema: board, view.
pub fn schema() -> Value {
    object(
        &[
            ("board", string("Board name")),
            ("view", choice("What to draw", &["pcb", "schematic"])),
        ],
        &["board", "view"],
    )
}

/// Standard base64 with padding.
fn base64(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let n = chunk
            .iter()
            .enumerate()
            .fold(0u32, |acc, (i, b)| acc | (u32::from(*b) << (16 - 8 * i)));
        for i in 0..4 {
            match i <= chunk.len() {
                true => out.push(ALPHABET[(n >> (18 - 6 * i) & 63) as usize] as char),
                false => out.push('='),
            }
        }
    }
    out
}

fn png(svg: &str) -> Option<Vec<u8>> {
    let mut child = Command::new("magick")
        .args(["svg:-", "png:-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;
    let mut stdin = child.stdin.take()?;
    let input = svg.as_bytes().to_vec();
    let writer = std::thread::spawn(move || stdin.write_all(&input));
    let output = child.wait_with_output().ok()?;
    let _ = writer.join();
    (output.status.success() && !output.stdout.is_empty()).then_some(output.stdout)
}

fn draw(project: &Project, board: &str, view: &str) -> Result<String, String> {
    let target = report::select(project, Some(board))?.remove(0);
    let (path, source) = load_circuit_source(&target.entry).map_err(|e| e.to_string())?;
    if view == "pcb" {
        return Ok(svg::render_pcb_svg(&source));
    }
    let circuit = parser::parse(&source, &path).map_err(|e| e.to_string())?;
    Ok(svg::render_schematic_svg(
        &circuit,
        &Netlist::from_circuit(&circuit),
    ))
}

fn rendered(project: &Project, args: &Value) -> Result<Outcome, Outcome> {
    let board = required_str(args, "board")?;
    let view = required_str(args, "view")?;
    if !["pcb", "schematic"].contains(&view) {
        return Err(Outcome::fail("view must be pcb or schematic"));
    }
    let svg = draw(project, board, view).map_err(Outcome::fail)?;
    let outcome = Outcome::ok(json!({"board": board, "view": view, "svg": svg}));
    Ok(match png(&svg) {
        Some(bytes) => outcome
            .with_extra(json!({"type": "image", "mimeType": "image/png", "data": base64(&bytes)})),
        None => outcome,
    })
}

/// Draws the requested view.
pub fn run(session: &mut Session, args: &Value) -> Outcome {
    rendered(&session.project(), args).unwrap_or_else(|outcome| outcome)
}
