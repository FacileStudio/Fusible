//! `mask_sliver`: the solder mask web between two pad openings.
//!
//! Each opening is the pad grown by the Gerber writer's default mask
//! expansion. A web narrower than `min_mask_dam` is removed by the fab, so
//! the pads share one opening and solder can bridge them. Openings that
//! already merge have no web and are left to the clearance rules. Findings
//! fold into one warning per part.

use std::collections::BTreeMap;

use super::index::Index;
use super::limit::{at, declared, violation, Gate};
use super::silk::{pads, Item};
use crate::config::{DesignRules, FabProfile};
use crate::gerber::layers::DEFAULT_MASK_MARGIN_MM;
use crate::linter::rules::copper::Shape;
use crate::linter::rules::physics::mm;
use crate::linter::{Category, Diagnostic, LintContext, Rule, RuleMeta, Severity};

/// Checks the mask web between neighbouring pads.
pub struct MaskSliverRule;

const META: RuleMeta = RuleMeta {
    id: "mask_sliver",
    category: Category::Dfm,
    summary: "The solder mask web between neighbouring pads meets the fab's minimum dam.",
    standard: Some("fab capability profile"),
    source_url: None,
    default_severity: Severity::Warning,
};

struct Sliver<'a> {
    web: f64,
    count: usize,
    pads: Vec<&'a str>,
    first: &'a Item,
}

fn webs(copper: &[Item], min: f64) -> Vec<(usize, usize, f64)> {
    let index = Index::new(copper.iter().map(|p| &p.shape));
    let reach = min + 2.0 * DEFAULT_MASK_MARGIN_MM;
    let mut found = Vec::new();
    for (i, a) in copper.iter().enumerate() {
        let later = index.near(&a.shape, reach).into_iter().filter(|&j| j > i);
        for j in later.filter(|&j| copper[j].layer == a.layer && copper[j].id != a.id) {
            let web = a.shape.gap(&copper[j].shape) - 2.0 * DEFAULT_MASK_MARGIN_MM;
            if web > 1e-9 && web < min - 1e-9 {
                found.push((i, j, web));
            }
        }
    }
    found
}

fn report(fab: &FabProfile, part: &str, s: &Sliver<'_>, min: f64) -> Diagnostic {
    let message = format!(
        "{part}: {} mask web(s) on {} below {} minimum {} mm, narrowest {} mm",
        s.count,
        s.first.layer,
        fab.name,
        mm(min),
        mm(s.web)
    );
    let (x, y) = match s.first.shape {
        Shape::Rect { c, .. } => c,
        Shape::Seg { a, .. } => a,
    };
    let elements = s.pads.iter().map(|p| p.to_string()).collect();
    violation(
        &META,
        fab,
        message,
        elements,
        at(x, y, Some(&s.first.layer)),
    )
}

fn scan(ctx: &LintContext<'_>, fab: &FabProfile, min: f64) -> Vec<Diagnostic> {
    let copper = pads(ctx);
    let mut parts: BTreeMap<String, Sliver<'_>> = BTreeMap::new();
    for (i, j, web) in webs(&copper, min) {
        let part = copper[i]
            .owner
            .clone()
            .unwrap_or_else(|| copper[i].id.clone());
        let entry = parts.entry(part).or_insert(Sliver {
            web,
            count: 0,
            pads: Vec::new(),
            first: &copper[i],
        });
        entry.web = entry.web.min(web);
        entry.count += 1;
        entry
            .pads
            .extend([copper[i].id.as_str(), copper[j].id.as_str()]);
    }
    for sliver in parts.values_mut() {
        sliver.pads.sort_unstable();
        sliver.pads.dedup();
    }
    parts
        .iter()
        .map(|(part, s)| report(fab, part, s, min))
        .collect()
}

impl Rule for MaskSliverRule {
    fn meta(&self) -> &'static RuleMeta {
        &META
    }

    fn enabled(&self, design: Option<&DesignRules>) -> bool {
        declared(design, |f| f.min_mask_dam)
    }

    fn check(&self, ctx: &LintContext<'_>) -> Vec<Diagnostic> {
        let gate = Gate {
            meta: &META,
            key: "min_mask_dam",
            pick: |f| f.min_mask_dam,
            relevant: ctx.circuit.pads().next().is_some(),
            ctx,
        };
        gate.run(|fab, min| scan(ctx, fab, min))
    }
}
