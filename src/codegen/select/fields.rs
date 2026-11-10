//! Descriptor strings of a chip and the matching rules over them.

use crate::parser::{Circuit, Component};
use crate::silicon::find_target;

pub(super) fn descriptors(circuit: &Circuit, component: &Component) -> Vec<String> {
    let mut fields: Vec<String> = [
        component.name.as_deref(),
        component.kind.as_deref(),
        component.component_id.as_deref(),
    ]
    .into_iter()
    .flatten()
    .map(str::to_string)
    .collect();
    fields.extend(manufacturer_part_number(circuit, component));
    fields
}

pub(super) fn manufacturer_part_number(circuit: &Circuit, component: &Component) -> Option<String> {
    let id = component.component_id.as_deref()?;
    let parts = circuit.model().parts.iter();
    parts
        .filter(|part| part.source_component_id.as_deref() == Some(id))
        .find_map(|part| part.manufacturer_part_number.clone())
}

fn normalize(text: &str) -> String {
    text.chars()
        .filter(|c| !matches!(c, '-' | '_' | ' '))
        .map(|c| c.to_ascii_lowercase())
        .collect()
}

pub(super) fn matches_target(field: &str, target: &str) -> bool {
    let target = normalize(target);
    !target.is_empty() && normalize(field).contains(&target)
}

pub(super) fn names_database_part(field: &str) -> bool {
    let clean = normalize(field);
    (1..=clean.len())
        .filter(|&end| clean.is_char_boundary(end))
        .any(|end| find_target(&clean[..end]).is_some())
}
