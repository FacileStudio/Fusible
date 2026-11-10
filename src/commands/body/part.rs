//! Code parts: a TypeScript module on the OpenCascade kernel, modeled against the real board.

use std::path::{Path, PathBuf};

use serde::Serialize;

use super::jobs::code_board;
use super::kit_data::{build_kit, Kit};
use super::mech_rules_data::clearance_gap;
use super::part_features::PartReport;
use super::settings::resolve_settings;
use super::spec::Output;
use super::{fuse_gate, run_capture, BodyError, Gate};
use crate::bom::resolve_material;
use crate::commands::check::load_circuit_source;
use crate::commands::target::BoardTarget;
use crate::config::{BodyConfig, EnclosureConfig};
use crate::project::Project;

#[derive(Serialize)]
struct PartJob<'a> {
    source: PathBuf,
    kit: Option<&'a Kit>,
    material: String,
    process: String,
    gap: f64,
    outputs: Vec<Output>,
}

/// Where the features of the last build of `name` are kept, next to its model files.
pub(crate) fn features_path(root: &Path, name: &str, body: &BodyConfig) -> PathBuf {
    let dir = body.enclosure.out_dir.clone();
    root.join(dir.unwrap_or_else(|| PathBuf::from("dist/bodies")))
        .join(format!("{name}.features.json"))
}

/// Runs the module of `body` against the `circuit` JSON of its board, and writes `outputs`.
///
/// `circuit` is the Circuit JSON and the manifest name of the board, or `None` for a part with
/// no board. The report holds the features the module recorded and the size of its shape.
pub(crate) fn build_part(
    body: &BodyConfig,
    root: &Path,
    circuit: Option<(&str, Option<String>)>,
    outputs: Vec<Output>,
) -> Result<PartReport, BodyError> {
    let source = body.source.clone().unwrap_or_default();
    let kit = circuit
        .map(|(json, name)| build_kit(json, name))
        .transpose()?;
    let (material, process) = resolve_material(body.material.as_deref(), body.process.as_deref());
    let job = PartJob {
        source: std::path::absolute(root.join(source))?,
        kit: kit.as_ref(),
        gap: clearance_gap(&process),
        material,
        process,
        outputs,
    };
    let payload = serde_json::to_vec(&job).map_err(std::io::Error::other)?;
    let stdout = run_capture("part.ts", &payload)?;
    PartReport::from_output(&stdout).ok_or_else(|| BodyError::GeneratorFailed {
        command: "bun part.ts".into(),
        code: None,
        stderr: "the part module printed no report".into(),
    })
}

/// Builds the code part `name` into the output directory of the manifest or of `overrides`.
///
/// The board is linted first unless `skip_lint` is set. The recorded features are cached next
/// to the models for `fusible check`.
pub(crate) fn build_code(
    name: &str,
    body: &BodyConfig,
    board: Option<&BoardTarget>,
    overrides: &EnclosureConfig,
    gate: Gate,
) -> Result<(), BodyError> {
    let settings = resolve_settings(overrides.clone(), body.enclosure.clone())?;
    let circuit = board
        .map(|b| fuse_gate(gate, b).map(|json| (json, b.name.clone())))
        .transpose()?;
    let out_dir = std::path::absolute(&settings.out_dir)?;
    std::fs::create_dir_all(&out_dir)?;
    let outputs: Vec<Output> = settings
        .formats
        .iter()
        .map(|&format| Output {
            format,
            path: out_dir.join(format!("{name}.{format}")),
        })
        .collect();
    println!("🧊 Building code part `{name}` with OpenCascade...");
    let circuit_ref = circuit.as_ref().map(|(json, n)| (json.as_str(), n.clone()));
    let report = build_part(body, Path::new("."), circuit_ref, outputs.clone())?;
    write_report(&out_dir, name, &report)?;
    for output in outputs {
        println!(
            "✅ Body {name} (code, {}) -> {}",
            output.format,
            output.path.display()
        );
    }
    Ok(())
}

/// Builds the GLB of a code part without a fuse check, for the dev studio and the assembly.
pub(crate) fn code_model(
    body: &BodyConfig,
    root: &Path,
    board: Option<&BoardTarget>,
    format: &'static str,
    path: PathBuf,
) -> Result<PartReport, BodyError> {
    let circuit = board
        .map(|b| load_circuit_source(&b.entry).map(|(_, json)| (json, b.name.clone())))
        .transpose()?;
    let circuit_ref = circuit.as_ref().map(|(json, n)| (json.as_str(), n.clone()));
    build_part(body, root, circuit_ref, vec![Output { format, path }])
}

/// Caches the features of the last build of `name` in `out_dir`, for `fusible check`.
pub(super) fn write_report(out_dir: &Path, name: &str, report: &PartReport) -> std::io::Result<()> {
    let text = serde_json::to_string_pretty(report).map_err(std::io::Error::other)?;
    std::fs::write(out_dir.join(format!("{name}.features.json")), text + "\n")
}

/// Reads the features cached by the last build of `name`.
pub(crate) fn read_report(root: &Path, name: &str, body: &BodyConfig) -> Option<PartReport> {
    let text = std::fs::read_to_string(features_path(root, name, body)).ok()?;
    serde_json::from_str(&text).ok()
}

/// Builds the code part `name` as a GLB at `output` without a fuse check, for the dev studio.
pub(crate) fn code_glb(
    project: &Project,
    name: &str,
    root: &Path,
    output: &Path,
) -> Result<(), BodyError> {
    let body = project
        .config
        .bodies
        .get(name)
        .ok_or_else(|| BodyError::UnknownBody {
            name: name.to_string(),
            help: "add it to `bodies:` in fusible.yml".into(),
        })?;
    let board = code_board(project, body)?;
    code_model(body, root, board.as_ref(), "glb", output.to_path_buf()).map(|_| ())
}

/// Builds a code part as one STEP file at `path` from the Circuit JSON of its board, for the assembly.
pub(crate) fn code_step(
    body: &BodyConfig,
    root: &Path,
    circuit: Option<(&str, &str)>,
    path: PathBuf,
) -> Result<(), BodyError> {
    let circuit = circuit.map(|(json, board)| (json, Some(board.to_string())));
    let outputs = vec![Output {
        format: "step",
        path,
    }];
    build_part(body, root, circuit, outputs).map(|_| ())
}
