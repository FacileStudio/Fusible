//! Copper pour regions with clear-polarity cutouts around foreign copper and the board edge.

use super::copper::CopperLayer;
use super::elements::PcbModel;
use super::geometry::{Feature, PadShape};
use super::nets::same_net;
use super::pour_shape::pour_rings;
use super::shapes::trace_segment;
use super::{invalid, GerberError};
use crate::gerber::apertures::Aperture;

pub(super) const POUR_CLEARANCE_MM: f64 = 0.2;

impl PcbModel {
    /// Returns the pour features of one copper layer: dark fills first, then clear cutouts per pour.
    /// Thermal relief is not generated: same-net pads and holes connect solidly.
    pub(super) fn pour_features(&self, layer: CopperLayer) -> Result<Vec<Feature>, GerberError> {
        let mut features = Vec::new();
        for (i, pour) in self.pours.iter().enumerate() {
            let on_layer =
                CopperLayer::parse(pour.layer.as_deref()).map_err(invalid("pcb_copper_pour", i))?;
            if on_layer != layer {
                continue;
            }
            let (outer, holes) = pour_rings(pour).map_err(invalid("pcb_copper_pour", i))?;
            let net = self.nets.source_net(pour.source_net_id.as_deref());
            features.push(Feature::Region(outer.clone()));
            features.push(Feature::Polarity(false));
            features.extend(holes.into_iter().map(Feature::Region));
            features.extend(self.cutouts(layer, &net)?);
            features.extend(self.edge_clip(&outer)?);
            features.push(Feature::Polarity(true));
        }
        Ok(features)
    }

    fn cutouts(
        &self,
        layer: CopperLayer,
        net: &Option<String>,
    ) -> Result<Vec<Feature>, GerberError> {
        let mut features = self.smtpad_features(|pad| {
            let on = CopperLayer::parse(pad.layer.as_deref())? == layer;
            let foreign = !same_net(&self.nets.port_net(pad.pcb_port_id.as_deref()), net);
            Ok((on && foreign).then_some(POUR_CLEARANCE_MM))
        })?;
        features.extend(self.plated_ring_features(|hole| {
            let foreign = !same_net(&self.nets.port_net(hole.pcb_port_id.as_deref()), net);
            Ok(foreign.then_some(POUR_CLEARANCE_MM))
        })?);
        features.extend(self.foreign_vias_and_traces(layer, net)?);
        features.extend(self.hole_clearances()?);
        Ok(features)
    }

    fn foreign_vias_and_traces(
        &self,
        layer: CopperLayer,
        net: &Option<String>,
    ) -> Result<Vec<Feature>, GerberError> {
        let mut features = Vec::new();
        for (i, via) in self.via_sites()?.into_iter().enumerate() {
            if !same_net(&via.net, net) {
                let pad = PadShape::Circle { d: via.outer };
                features.push(
                    pad.feature(via.at, POUR_CLEARANCE_MM)
                        .map_err(invalid("via", i))?,
                );
            }
        }
        features.extend(self.foreign_traces(layer, net)?);
        Ok(features)
    }

    fn foreign_traces(
        &self,
        layer: CopperLayer,
        net: &Option<String>,
    ) -> Result<Vec<Feature>, GerberError> {
        let mut features = Vec::new();
        let foreign = self
            .traces
            .iter()
            .enumerate()
            .filter(|(_, t)| !same_net(&self.nets.trace_net(t), net));
        for (i, trace) in foreign {
            for win in trace.route.windows(2) {
                let seg = trace_segment(&win[0], &win[1]).map_err(invalid("pcb_trace", i))?;
                let ap = Aperture::circle(seg.width + 2.0 * POUR_CLEARANCE_MM);
                if seg.layer == layer {
                    features.push(Feature::Stroke(
                        ap.map_err(invalid("pcb_trace", i))?,
                        seg.from,
                        seg.to,
                    ));
                }
            }
        }
        Ok(features)
    }
}
