//! Pick-and-place CSV from the PCB components of a Circuit JSON.

use std::collections::HashMap;

use serde_json::Value;

use super::fab_files::invalid;
use super::ExportError;
use crate::bom;

const PNP_HEADER: &str = "designator,value,package,x,y,rotation,side\n";

/// Renders `pnp.csv`: one row per `pcb_component`, positioned at its `center`.
pub(super) fn pnp_csv(circuit_json: &str) -> Result<String, ExportError> {
    let elements: Vec<Value> = serde_json::from_str(circuit_json).map_err(invalid)?;
    let names = index(&elements, "source_component", "source_component_id", "name");
    let packages = index(
        &elements,
        "cad_component",
        "pcb_component_id",
        "footprinter_string",
    );
    let values = values_by_designator(circuit_json)?;
    let mut out = String::from(PNP_HEADER);
    for el in elements
        .iter()
        .filter(|el| text(el, "type") == Some("pcb_component"))
    {
        let source = text(el, "source_component_id").unwrap_or_default();
        let name = names
            .get(source)
            .cloned()
            .unwrap_or_else(|| source.to_string());
        let package = packages.get(text(el, "pcb_component_id").unwrap_or_default());
        let value = values.get(&name).cloned().unwrap_or_default();
        out.push_str(&row(
            el,
            [name, value, package.cloned().unwrap_or_default()],
        ));
    }
    Ok(out)
}

fn row(el: &Value, labels: [String; 3]) -> String {
    let coord = |axis: &str| {
        el.pointer(&format!("/center/{axis}"))
            .and_then(Value::as_f64)
            .unwrap_or(0.0)
    };
    let rotation = el.get("rotation").and_then(Value::as_f64).unwrap_or(0.0);
    let side = text(el, "layer").unwrap_or("top");
    let labels = labels.map(|c| quote(&c)).join(",");
    format!(
        "{labels},{:.4},{:.4},{rotation:.2},{side}\n",
        coord("x"),
        coord("y")
    )
}

fn text<'a>(el: &'a Value, key: &str) -> Option<&'a str> {
    el.get(key).and_then(Value::as_str)
}

fn index<'a>(elements: &'a [Value], kind: &str, key: &str, val: &str) -> HashMap<&'a str, String> {
    let of_kind = elements.iter().filter(|el| text(el, "type") == Some(kind));
    of_kind
        .filter_map(|el| Some((text(el, key)?, text(el, val)?.to_string())))
        .collect()
}

fn values_by_designator(circuit_json: &str) -> Result<HashMap<String, String>, ExportError> {
    let parts = bom::detect(circuit_json, None).map_err(invalid)?;
    let mut map = HashMap::new();
    for part in parts {
        let value = part.value.or(part.mpn).unwrap_or_default();
        map.extend(part.designators.into_iter().map(|d| (d, value.clone())));
    }
    Ok(map)
}

fn quote(cell: &str) -> String {
    if cell.contains([',', '"', '\n']) {
        format!("\"{}\"", cell.replace('"', "\"\""))
    } else {
        cell.to_string()
    }
}
