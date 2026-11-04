//! What the Circuit JSON says about each part beyond its footprint: connector or not, LCSC id.

use std::collections::{HashMap, HashSet};

use super::BodyError;
use crate::gerber::layers::PcbComponentElm;

/// Words in a part kind that make it a connector with an opening in the wall.
const CONNECTOR_WORDS: &[&str] = &["connector", "usb", "jack", "header", "socket"];

/// The connectors of a circuit, and the LCSC ids of its parts.
#[derive(Debug, Default)]
pub(super) struct PartFacts {
    connectors: HashSet<String>,
    lcsc: HashMap<String, Vec<String>>,
}

fn text<'a>(element: &'a serde_json::Value, key: &str) -> Option<&'a str> {
    element[key].as_str()
}

fn is_connector(element: &serde_json::Value) -> bool {
    let kind = ["ftype", "kind"]
        .iter()
        .find_map(|key| text(element, key))
        .unwrap_or_default()
        .to_lowercase();
    CONNECTOR_WORDS.iter().any(|w| kind.contains(w)) || !element["cable_insertion_center"].is_null()
}

fn lcsc_ids(element: &serde_json::Value) -> Vec<String> {
    let listed = element["supplier_part_numbers"]["lcsc"].as_array();
    let listed = listed.into_iter().flatten().filter_map(|v| v.as_str());
    let mpn = text(element, "manufacturer_part_number");
    listed.chain(mpn).map(str::to_string).collect()
}

impl PartFacts {
    /// Reads the connectors and LCSC ids of the source and pcb components in `json`.
    pub(super) fn parse(json: &str) -> Result<Self, BodyError> {
        let elements: Vec<serde_json::Value> =
            serde_json::from_str(json).map_err(|e| BodyError::InvalidCircuit(e.to_string()))?;
        let mut facts = Self::default();
        for element in &elements {
            let kind = text(element, "type");
            let id = if kind == Some("source_component") {
                text(element, "source_component_id").or_else(|| text(element, "component_id"))
            } else if kind == Some("pcb_component") {
                text(element, "pcb_component_id")
            } else {
                continue;
            };
            let Some(id) = id else { continue };
            if is_connector(element) {
                facts.connectors.insert(id.to_string());
            }
            if kind == Some("source_component") {
                facts.lcsc.insert(id.to_string(), lcsc_ids(element));
            }
        }
        Ok(facts)
    }

    /// Whether `component` is a connector that needs an opening in the wall.
    pub(super) fn is_connector(&self, component: &PcbComponentElm) -> bool {
        [&component.source_component_id, &component.pcb_component_id]
            .into_iter()
            .flatten()
            .any(|id| self.connectors.contains(id))
    }

    /// The LCSC ids and manufacturer part number of `component`.
    pub(super) fn part_ids(&self, component: &PcbComponentElm) -> &[String] {
        component
            .source_component_id
            .as_ref()
            .and_then(|id| self.lcsc.get(id))
            .map_or(&[], Vec::as_slice)
    }
}
