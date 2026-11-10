//! Fabrication bundle: gerbers, Circuit JSON, bill of materials and pick-and-place.

use std::path::Path;

use super::ExportError;
use crate::bom::{self, BomFile};

/// Writes `gerbers.zip`, `circuit.json`, `bom.csv` and `pnp.csv` into `dir`; nothing is written
/// when gerber generation fails.
///
/// `manifest` is the project's `fusible.yml`; the `bom.yml` beside it prices the BOM.
pub fn write_fab_bundle(
    circuit_json: &str,
    title: &str,
    dir: &Path,
    manifest: Option<&Path>,
) -> Result<(), ExportError> {
    let bom = bom_csv(circuit_json, manifest)?;
    let pnp = super::pnp::pnp_csv(circuit_json)?;
    crate::gerber::export_fab_bundle(circuit_json, title, dir)?;
    std::fs::write(dir.join("bom.csv"), bom)?;
    std::fs::write(dir.join("pnp.csv"), pnp)?;
    Ok(())
}

pub(super) fn invalid(message: impl ToString) -> ExportError {
    ExportError::Io(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        message.to_string(),
    ))
}

pub(super) fn bom_csv(circuit_json: &str, manifest: Option<&Path>) -> Result<String, ExportError> {
    let parts = bom::detect(circuit_json, None).map_err(invalid)?;
    let file = match manifest {
        Some(manifest) => BomFile::load(&bom::bom_path(manifest)).map_err(invalid)?,
        None => BomFile::default(),
    };
    Ok(bom::csv(&bom::build(parts, &[], &file)))
}
