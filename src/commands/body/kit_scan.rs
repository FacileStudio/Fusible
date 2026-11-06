//! The per-component scan of a board: names, heights, connector facts, openings.

use std::collections::HashMap;

use super::edges::edge_contacts;
use super::kit_data::{KitComponent, KitOpening};
use super::kit_levels::{component_heights, level_of, verified_top};
use super::kit_parts::PartFacts;
use super::BodyError;
use crate::gerber::layers::{PcbComponentElm, PcbPoint};

pub(super) fn component_names(json: &str) -> Result<HashMap<String, String>, BodyError> {
    let elements: Vec<serde_json::Value> =
        serde_json::from_str(json).map_err(|e| BodyError::InvalidCircuit(e.to_string()))?;
    let text = |e: &serde_json::Value, key: &str| e[key].as_str().map(str::to_string);
    Ok(elements
        .iter()
        .filter(|e| e["type"] == "source_component")
        .filter_map(|e| {
            let id = text(e, "source_component_id").or_else(|| text(e, "component_id"))?;
            Some((id.clone(), text(e, "name").unwrap_or(id)))
        })
        .collect())
}

fn side(normal: [f64; 2]) -> &'static str {
    let horizontal = normal[0].abs() >= normal[1].abs();
    match (horizontal, normal[0] >= 0.0, normal[1] >= 0.0) {
        (true, true, _) => "+x",
        (true, false, _) => "-x",
        (false, _, true) => "+y",
        (false, _, false) => "-y",
    }
}

fn label(component: &PcbComponentElm, names: &HashMap<String, String>) -> String {
    let id = component.source_component_id.clone();
    id.as_ref()
        .and_then(|id| names.get(id).cloned())
        .or(id)
        .or_else(|| component.pcb_component_id.clone())
        .unwrap_or_else(|| "component".into())
}

/// What one board scan shares between its components.
pub(super) struct Scan<'a> {
    outline: &'a [PcbPoint],
    names: HashMap<String, String>,
    heights: HashMap<String, (f64, f64)>,
    facts: PartFacts,
    half: f64,
}

/// The bottom and height of `c`, with a verified top applied to a top-side part.
fn level(c: &PcbComponentElm, scan: &Scan) -> Option<(f64, f64)> {
    let measured = level_of(c, &scan.heights);
    let on_top = c.layer.as_deref().unwrap_or("top") == "top";
    let Some(top) = verified_top(scan.facts.part_ids(c)).filter(|_| on_top) else {
        return measured;
    };
    let bottom = measured.map_or(scan.half, |(z, _)| z);
    Some((bottom, scan.half + top - bottom))
}

/// A component's kit entry and the wall openings it needs, or `None` without a footprint.
///
/// Only a connector gets openings; any other part at the edge stays in `components` alone.
pub(super) fn entry(c: &PcbComponentElm, scan: &Scan) -> Option<(KitComponent, Vec<KitOpening>)> {
    let (center, w, h) = (c.center?, c.width?, c.height?);
    let name = label(c, &scan.names);
    let contacts = edge_contacts(c, scan.outline);
    let wanted = scan.facts.is_connector(c);
    let openings: Vec<KitOpening> = contacts
        .iter()
        .filter(|_| wanted)
        .map(|k| KitOpening {
            component: name.clone(),
            edge: k.edge,
            side: side(k.normal),
            normal: k.normal,
            from: k.span[0],
            to: k.span[1],
        })
        .collect();
    let level = level(c, scan);
    let component = KitComponent {
        z: level.map(|(z, _)| z),
        height: level.map(|(_, h)| h),
        layer: c.layer.clone().unwrap_or_else(|| "top".into()),
        touches_edge: !contacts.is_empty(),
        x: center.x,
        y: center.y,
        name,
        w,
        h,
    };
    Some((component, openings))
}

/// The scan of the board in `json`: its outline, thickness and per-component facts.
pub(super) fn scan_of<'a>(
    json: &str,
    names: HashMap<String, String>,
    outline: &'a [PcbPoint],
    thickness: f64,
) -> Result<Scan<'a>, BodyError> {
    Ok(Scan {
        outline,
        names,
        heights: component_heights(json)?,
        facts: PartFacts::parse(json)?,
        half: thickness / 2.0,
    })
}
