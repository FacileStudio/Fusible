//! Pour clearances that depend on the board: outline clip and non-plated holes.

use super::elements::PcbModel;
use super::geometry::{Drill, Feature, PadShape};
use super::holes::hole_drill;
use super::invalid;
use super::pour::POUR_CLEARANCE_MM;
use super::pour_shape::outside_region;
use super::GerberError;
use crate::gerber::apertures::Aperture;

impl PcbModel {
    /// Returns the clear features that keep a pour inside every board outline.
    pub(super) fn edge_clip(&self, pour: &[[f64; 2]]) -> Result<Vec<Feature>, GerberError> {
        let rings: Vec<Vec<[f64; 2]>> = self
            .boards
            .iter()
            .map(|b| b.outline_points().iter().map(|p| [p.x, p.y]).collect())
            .filter(|r: &Vec<[f64; 2]>| r.len() >= 3 && r.iter().flatten().all(|v| v.is_finite()))
            .collect();
        if rings.is_empty() {
            return Ok(Vec::new());
        }
        let ap = Aperture::circle(2.0 * POUR_CLEARANCE_MM).map_err(invalid("pcb_board", 0))?;
        let mut features = vec![Feature::Region(outside_region(&rings, pour))];
        for ring in &rings {
            let closing = [ring[ring.len() - 1], ring[0]];
            features.extend(
                ring.windows(2)
                    .map(|w| Feature::Stroke(ap.clone(), w[0], w[1])),
            );
            features.push(Feature::Stroke(ap.clone(), closing[0], closing[1]));
        }
        Ok(features)
    }

    /// Returns the clearance rings around every non-plated hole.
    pub(super) fn hole_clearances(&self) -> Result<Vec<Feature>, GerberError> {
        let mut features = Vec::new();
        for (i, hole) in self.holes.iter().enumerate() {
            let shape = match hole_drill(hole).map_err(invalid("pcb_hole", i))? {
                Drill::Round { d, .. } => PadShape::Circle { d },
                Drill::Slot { w, h, rot, .. } => PadShape::Pill { w, h, rot },
            };
            let at = [hole.x, hole.y];
            features.push(
                shape
                    .feature(at, POUR_CLEARANCE_MM)
                    .map_err(invalid("pcb_hole", i))?,
            );
        }
        Ok(features)
    }
}
