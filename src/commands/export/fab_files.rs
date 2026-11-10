//! Fabrication bundle: gerbers, Circuit JSON, bill of materials, centroid,
//! test netlist and README.

use std::path::Path;

use super::ExportError;
use crate::bom::{self, BomFile};
use crate::config::Date;
use crate::netlist::Netlist;

/// Writes `gerbers.zip`, `circuit.json`, `bom.csv`, `centroid.csv`,
/// `netlist.ipc` and `README.txt` into `dir`; nothing is written when the
/// Circuit JSON cannot be read or gerber generation fails.
///
/// `manifest` is the project's `fusible.yml`; the `bom.yml` beside it prices the BOM.
pub fn write_fab_bundle(
    circuit_json: &str,
    title: &str,
    dir: &Path,
    manifest: Option<&Path>,
) -> Result<(), ExportError> {
    let bom = bom_csv(circuit_json, manifest)?;
    let centroid = crate::fab::centroid_csv(circuit_json).map_err(invalid)?;
    let netlist = ipc_netlist(circuit_json, title)?;
    let layers = crate::gerber::export_fab_bundle(circuit_json, title, dir)?;
    let readme = crate::fab::readme(title, Date::today(), layers.iter().map(String::as_str));
    std::fs::write(dir.join("bom.csv"), bom)?;
    std::fs::write(dir.join("centroid.csv"), centroid)?;
    std::fs::write(dir.join("netlist.ipc"), netlist)?;
    std::fs::write(dir.join("README.txt"), readme)?;
    Ok(())
}

fn ipc_netlist(circuit_json: &str, title: &str) -> Result<String, ExportError> {
    let circuit = crate::parser::parse(circuit_json, Path::new("circuit.json"))
        .map_err(|e| invalid(e.to_string()))?;
    let netlist = Netlist::from_circuit(&circuit);
    Ok(crate::fab::ipc356(&circuit, &netlist, title))
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
