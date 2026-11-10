//! `annular_ring`: copper left around each plated drill.
//!
//! Plated holes are held to `min_annular`, vias to `min_via_annular`; the
//! fab publishes the two apart. A ring is the pad edge to the drill edge on
//! the narrowest side, less any drill offset.

use super::limit::{at, profile, violation};
use crate::config::FabProfile;
use crate::linter::rules::physics::mm;
use crate::linter::{Category, Diagnostic, LintContext, Rule, RuleMeta, Severity};
use crate::parser::model::PcbPlatedHole;

const EPSILON: f64 = 1e-9;

/// Checks the annular ring of every plated hole and via.
pub struct AnnularRingRule;

const META: RuleMeta = RuleMeta {
    id: "annular_ring",
    category: Category::Dfm,
    summary: "Plated holes and vias keep the fab's minimum copper ring around the drill.",
    standard: Some("fab capability profile"),
    source_url: None,
    default_severity: Severity::Error,
};

fn pair(d: Option<f64>, w: Option<f64>, h: Option<f64>) -> Option<(f64, f64)> {
    d.map(|d| (d, d)).or_else(|| w.zip(h))
}

/// The ring of a plated hole, or `None` when its sizes are missing.
fn plated_ring(el: &PcbPlatedHole) -> Option<f64> {
    let (hw, hh) = pair(el.hole_diameter, el.hole_width, el.hole_height)?;
    let outer = pair(el.outer_diameter, el.outer_width, el.outer_height);
    let (pw, ph) = outer.or_else(|| el.rect_pad_width.zip(el.rect_pad_height))?;
    let turned = el
        .rect_ccw_rotation
        .is_some_and(|r| (r / 90.0).round() as i64 % 2 != 0);
    let (pw, ph) = if turned && el.outer_diameter.is_none() {
        (ph, pw)
    } else {
        (pw, ph)
    };
    let offset = el
        .hole_offset_x
        .unwrap_or(0.0)
        .abs()
        .max(el.hole_offset_y.unwrap_or(0.0).abs());
    Some(((pw - hw).min(ph - hh)) / 2.0 - offset)
}

struct Ring<'a> {
    what: &'static str,
    id: &'a str,
    ring: f64,
    xy: (f64, f64),
}

fn report(fab: &FabProfile, r: Ring<'_>, min: f64) -> Option<Diagnostic> {
    (r.ring < min - EPSILON).then(|| {
        let message = format!(
            "{} {} annular ring {} mm is below {} minimum {} mm",
            r.what,
            r.id,
            mm(r.ring.max(0.0)),
            fab.name,
            mm(min)
        );
        let location = at(r.xy.0, r.xy.1, None);
        violation(&META, fab, message, vec![r.id.to_string()], location)
    })
}

fn vias(ctx: &LintContext<'_>, fab: &FabProfile) -> Vec<Diagnostic> {
    let Some(min) = fab.min_via_annular else {
        return Vec::new();
    };
    let model = ctx.circuit.model();
    let found = model.vias.iter().filter_map(|v| {
        let ring = Ring {
            what: "via",
            id: &v.pcb_via_id,
            ring: (v.outer_diameter - v.hole_diameter) / 2.0,
            xy: (v.x, v.y),
        };
        report(fab, ring, min)
    });
    found.collect()
}

fn holes(ctx: &LintContext<'_>, fab: &FabProfile) -> Vec<Diagnostic> {
    let model = ctx.circuit.model();
    let found = model.plated_holes.iter().filter_map(|h| {
        let ring = Ring {
            what: "plated hole",
            id: &h.pcb_plated_hole_id,
            ring: plated_ring(h)?,
            xy: (h.x, h.y),
        };
        report(fab, ring, fab.min_annular)
    });
    found.collect()
}

impl Rule for AnnularRingRule {
    fn meta(&self) -> &'static RuleMeta {
        &META
    }

    fn enabled(&self, design: Option<&crate::config::DesignRules>) -> bool {
        design.is_some_and(|d| d.fab.is_some())
    }

    fn check(&self, ctx: &LintContext<'_>) -> Vec<Diagnostic> {
        let Some(fab) = profile(ctx) else {
            return Vec::new();
        };
        holes(ctx, fab).into_iter().chain(vias(ctx, fab)).collect()
    }
}
