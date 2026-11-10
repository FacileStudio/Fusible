//! Designators and sheets of the typed `schematic_component` elements.

use crate::linter::LintContext;
use crate::parser::model::SchematicComponent;

fn non_empty(text: Option<&str>) -> Option<&str> {
    text.filter(|t| !t.is_empty())
}

/// The component's designator: its `refdes` or `name`, else the name of its
/// source component, else its source or schematic id, else `unknown`.
pub(super) fn resolve_refdes(component: &SchematicComponent, ctx: &LintContext<'_>) -> String {
    non_empty(component.refdes.as_deref())
        .or_else(|| non_empty(component.name.as_deref()))
        .or_else(|| {
            let id = component.source_component_id.as_deref()?;
            Some(source_component_name(ctx, id).unwrap_or(id))
        })
        .or(component.schematic_component_id.as_deref())
        .unwrap_or("unknown")
        .to_string()
}

/// The first non-empty name of the source components with id `id`.
fn source_component_name<'a>(ctx: &'a LintContext<'_>, id: &str) -> Option<&'a str> {
    ctx.circuit
        .components()
        .filter(|c| c.component_id.as_deref() == Some(id))
        .find_map(|c| non_empty(c.name.as_deref()))
}

/// True when the circuit spans more than one schematic sheet.
pub(super) fn detect_multiple_sheets(ctx: &LintContext<'_>) -> bool {
    let model = ctx.circuit.model();
    model.count("schematic_sheet") > 1 || model.schematic_sheet_ids.len() > 1
}
