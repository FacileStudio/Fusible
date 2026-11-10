//! The BOM line id of a typed component, as [`super::detect`] names it.

use super::detect::part_id;
use super::value::si;
use crate::parser::Component;

fn value(component: &Component) -> Option<String> {
    let p = &component.props;
    let shown = [
        &p.display_value,
        &p.display_resistance,
        &p.display_capacitance,
        &p.display_inductance,
    ];
    let shown = shown
        .into_iter()
        .flatten()
        .map(|s| s.trim())
        .find(|s| !s.is_empty());
    if let Some(shown) = shown {
        return Some(shown.to_string());
    }
    let numbers = [
        (p.resistance, "Ω"),
        (p.capacitance, "F"),
        (p.inductance, "H"),
    ];
    numbers
        .into_iter()
        .find_map(|(n, unit)| Some(si(n.filter(|n| *n != 0.0)?, unit)))
}

/// The id of the BOM line `component` lands on, the key `bom.yml` uses.
pub fn line_id(component: &Component) -> String {
    let id = component.component_id.as_deref().unwrap_or("component");
    let name = component.name.as_deref().unwrap_or(id);
    let kind = component.kind.as_deref().unwrap_or("component");
    let kind = kind.strip_prefix("simple_").unwrap_or(kind);
    let mpn = component.props.manufacturer_part_number.as_deref();
    let mpn = mpn.map(str::trim).filter(|m| !m.is_empty());
    part_id(kind, value(component).as_deref(), mpn, name)
}
