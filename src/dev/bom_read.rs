//! How `GET /api/bom` reads the BOM file without writing it.

use crate::bom::{BomError, BomFile};

/// The BOM file as a GET sees it: loaded when present, empty otherwise, and never written.
pub(super) fn file_for_read(path: &std::path::Path) -> Result<BomFile, BomError> {
    match path.exists() {
        true => BomFile::load_or_create(path),
        false => Ok(BomFile::default()),
    }
}
