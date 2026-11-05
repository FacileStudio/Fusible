//! Release check: every copper pour must reach its gerber layer.

use serde_json::Value;

use super::ReleaseError;
use crate::gerber::layers::CopperLayer;
use crate::gerber::GerberFiles;

/// Fails with the layer name when a `pcb_copper_pour` layer file has no `G36` region.
pub(super) fn verify(
    board: &str,
    circuit_json: &str,
    files: &GerberFiles,
) -> Result<(), ReleaseError> {
    let elements: Vec<Value> = serde_json::from_str(circuit_json).unwrap_or_default();
    let pours = elements
        .iter()
        .filter(|el| el.get("type").and_then(Value::as_str) == Some("pcb_copper_pour"));
    for layer in pours.filter_map(|el| el.get("layer").and_then(Value::as_str)) {
        let file = CopperLayer::parse(Some(layer)).map(CopperLayer::file_name);
        let has_region = file
            .as_ref()
            .ok()
            .and_then(|f| files.get(f))
            .is_some_and(|gbr| gbr.contains("G36*"));
        if !has_region {
            return Err(ReleaseError::MissingPour {
                board: board.to_string(),
                layer: layer.to_string(),
            });
        }
    }
    Ok(())
}
