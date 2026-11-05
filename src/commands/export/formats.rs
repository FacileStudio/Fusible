//! Direct single-format exports selected with `--format`.

use std::path::{Path, PathBuf};

use super::{extract_zip_archive, ExportError, ExportJob};

/// Exports `job` into one named format.
pub(super) fn run_format(job: &ExportJob, format: &str) -> Result<(), ExportError> {
    match format {
        "gerbers" | "gerber" => export_gerbers(job),
        "pcb-svg" | "pcb_svg" => export_pcb_svg(job),
        "kicad_project" | "kicad" => export_kicad_project(job),
        other => export_via_runner(job, other),
    }
}

/// Creates the parent directory of `dest` when it has one.
fn create_parent(dest: &Path) -> Result<(), ExportError> {
    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)?;
    }
    Ok(())
}

/// Writes a zip of native RS-274X gerbers.
fn export_gerbers(job: &ExportJob) -> Result<(), ExportError> {
    let circuit_json = job.circuit_json()?;
    let dest = job.output_or("gerbers.zip");
    create_parent(&dest)?;
    println!(
        "📦 Generating native RS-274X gerbers to {}...",
        dest.display()
    );
    let files = crate::gerber::generate_gerber_files(&circuit_json, &job.board_title)
        .map_err(std::io::Error::from)?;
    let zip_bytes = crate::gerber::create_gerber_zip(&files)?;
    std::fs::write(&dest, zip_bytes)?;
    println!("✅ Successfully exported gerbers to {}!", dest.display());
    Ok(())
}

/// Writes the native PCB layout SVG.
fn export_pcb_svg(job: &ExportJob) -> Result<(), ExportError> {
    let circuit_json = job.circuit_json()?;
    let dest = job.output_or("pcb.svg");
    create_parent(&dest)?;
    println!(
        "📦 Generating native PCB layout SVG to {}...",
        dest.display()
    );
    let svg = crate::svg::render_pcb_svg(&circuit_json);
    std::fs::write(&dest, svg)?;
    println!(
        "✅ Successfully exported PCB layout SVG to {}!",
        dest.display()
    );
    Ok(())
}

/// Exports a KiCad project through the runner and unpacks it.
fn export_kicad_project(job: &ExportJob) -> Result<(), ExportError> {
    let runner = job.runner()?;
    let dest = job.output_or("cad/kicad");
    std::fs::create_dir_all(&dest)?;
    let tmp_zip = dest.join(".kicad_export.zip");
    println!("📦 Exporting KiCad project to {}...", dest.display());
    runner.export(&job.entry, "kicad_zip", Some(&tmp_zip))?;
    extract_zip_archive(&tmp_zip, &dest)?;
    std::fs::remove_file(&tmp_zip)?;
    println!(
        "✅ Successfully exported KiCad project to {}!",
        dest.display()
    );
    Ok(())
}

/// Delegates any other format to the tscircuit runner.
fn export_via_runner(job: &ExportJob, format: &str) -> Result<(), ExportError> {
    let runner = job.runner()?;
    println!(
        "📦 Exporting \"{}\" to format: {}...",
        job.entry.display(),
        format
    );
    let dest = runner_output(job, format);
    runner.export(&job.entry, format, Some(&dest))?;
    println!("✅ Successfully exported {format} to {}!", dest.display());
    Ok(())
}

/// Returns the explicit output, or `<out_dir>/<entry stem>.<ext>` so exports land in `dist/`.
pub(super) fn runner_output(job: &ExportJob, format: &str) -> PathBuf {
    let ext = match format {
        "kicad_zip" => "zip",
        "schematic-svg" | "schematic_svg" => "schematic.svg",
        other => other,
    };
    let stem = job
        .entry
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("board");
    job.output_or(&format!("{stem}.{ext}"))
}
