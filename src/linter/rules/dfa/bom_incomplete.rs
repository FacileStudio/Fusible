//! `bom_incomplete`: every placed part names what to buy.
//!
//! A part counts as named when its component carries a
//! `manufacturerPartNumber` or a supplier part number, or when its line in
//! `bom.yml` sets `mpn`. Fiducials and parts marked `do_not_place` buy
//! nothing. All unnamed parts fold into one warning.

use std::collections::HashMap;

use crate::bom::line_id;
use crate::linter::rules::dfm::at;
use crate::linter::{Category, Diagnostic, LintContext, Rule, RuleMeta, Severity};
use crate::parser::Component;

const HELP: &str = "set manufacturerPartNumber on the part, or `mpn` on its line in bom.yml \
                    (line ids from `fusible bom -f json`)";

/// Reports placed parts with no part number anywhere.
pub struct BomIncompleteRule;

const META: RuleMeta = RuleMeta {
    id: "bom_incomplete",
    category: Category::Dfa,
    summary: "Every placed part has a part number on the component or in bom.yml.",
    standard: None,
    source_url: None,
    default_severity: Severity::Warning,
};

fn named(ctx: &LintContext<'_>, part: &Component) -> bool {
    let props = &part.props;
    let own = props
        .manufacturer_part_number
        .as_deref()
        .is_some_and(|m| !m.trim().is_empty());
    let supplier = props
        .supplier_part_numbers
        .values()
        .flatten()
        .any(|n| !n.trim().is_empty());
    let listed = ctx
        .design
        .is_some_and(|d| d.bom_parts.contains(&line_id(part)));
    own || supplier || listed
}

fn buys_nothing(part: &Component) -> bool {
    part.kind.as_deref().is_some_and(|k| k.contains("fiducial"))
}

/// A placed part with no part number, and where it sits.
struct Unnamed<'a> {
    part: &'a Component,
    at: (f64, f64),
    layer: &'a str,
}

fn unnamed<'a>(ctx: &LintContext<'a>) -> Vec<Unnamed<'a>> {
    let parts: HashMap<&str, &Component> = ctx
        .circuit
        .components()
        .filter_map(|c| Some((c.component_id.as_deref()?, c)))
        .collect();
    let placed = ctx.circuit.model().components.iter();
    let placed = placed.filter(|p| p.do_not_place != Some(true));
    let found = placed.filter_map(|p| {
        Some(Unnamed {
            part: parts.get(p.source_component_id.as_str())?,
            at: (p.center.x, p.center.y),
            layer: &p.layer,
        })
    });
    found
        .filter(|u| !buys_nothing(u.part) && !named(ctx, u.part))
        .collect()
}

fn report(missing: &[Unnamed<'_>]) -> Option<Diagnostic> {
    let first = missing.first()?;
    let names: Vec<&str> = missing
        .iter()
        .map(|u| {
            u.part
                .name
                .as_deref()
                .or(u.part.component_id.as_deref())
                .unwrap_or("?")
        })
        .collect();
    Some(Diagnostic {
        rule: META.id.to_string(),
        message: format!(
            "{} placed part(s) have no part number: {}",
            names.len(),
            names.join(", ")
        ),
        help: Some(HELP.into()),
        severity: META.default_severity,
        elements: missing
            .iter()
            .filter_map(|u| u.part.component_id.clone())
            .collect(),
        location: Some(at(first.at.0, first.at.1, Some(first.layer))),
        ..Default::default()
    })
}

impl Rule for BomIncompleteRule {
    fn meta(&self) -> &'static RuleMeta {
        &META
    }

    fn check(&self, ctx: &LintContext<'_>) -> Vec<Diagnostic> {
        report(&unnamed(ctx)).into_iter().collect()
    }
}
