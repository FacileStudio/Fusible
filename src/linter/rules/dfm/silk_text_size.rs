//! `silk_text_size`: legend text tall enough and lines wide enough to read.
//!
//! Text shorter than `min_silk_height` and drawn lines thinner than
//! `min_silk_stroke` come out unreadable. Each limit folds its findings into
//! one warning that lists every element and sits at the first one.

use super::limit::{at, declared, missing, profile, violation};
use crate::config::{DesignRules, FabProfile};
use crate::linter::rules::physics::mm;
use crate::linter::{Category, Diagnostic, LintContext, Rule, RuleMeta, Severity};
use crate::parser::model::Model;

/// Checks silkscreen text height and line width.
pub struct SilkTextSizeRule;

const META: RuleMeta = RuleMeta {
    id: "silk_text_size",
    category: Category::Dfm,
    summary: "Silkscreen text and lines meet the fab's minimum height and stroke.",
    standard: Some("fab capability profile"),
    source_url: None,
    default_severity: Severity::Warning,
};

struct Small {
    id: String,
    size: f64,
    at: (f64, f64),
    layer: String,
}

fn fold(fab: &FabProfile, what: &str, min: f64, small: Vec<Small>) -> Option<Diagnostic> {
    let first = small.first()?;
    let smallest = small.iter().map(|s| s.size).fold(f64::MAX, f64::min);
    let message = format!(
        "{} silkscreen {what}(s) below {} minimum {} mm, smallest {} mm ({} first)",
        small.len(),
        fab.name,
        mm(min),
        mm(smallest),
        first.id
    );
    let location = at(first.at.0, first.at.1, Some(&first.layer));
    let elements = small.iter().map(|s| s.id.clone()).collect();
    Some(violation(&META, fab, message, elements, location))
}

fn short_texts(model: &Model, min: f64) -> Vec<Small> {
    let short = model
        .silkscreen_texts
        .iter()
        .filter(|t| t.font_size < min - 1e-9);
    short
        .map(|t| Small {
            id: t.pcb_silkscreen_text_id.clone(),
            size: t.font_size,
            at: (t.anchor_position.x, t.anchor_position.y),
            layer: t.layer.clone(),
        })
        .collect()
}

fn thin_paths(model: &Model, min: f64) -> Vec<Small> {
    let drawn = model
        .silkscreen_paths
        .iter()
        .filter(|p| p.stroke_width > 0.0);
    let thin = drawn.filter(|p| p.stroke_width < min - 1e-9);
    thin.map(|p| Small {
        id: p.pcb_silkscreen_path_id.clone(),
        size: p.stroke_width,
        at: p.route.first().map_or((0.0, 0.0), |q| (q.x, q.y)),
        layer: p.layer.clone(),
    })
    .collect()
}

impl Rule for SilkTextSizeRule {
    fn meta(&self) -> &'static RuleMeta {
        &META
    }

    fn enabled(&self, design: Option<&DesignRules>) -> bool {
        declared(design, |f| f.min_silk_height) || declared(design, |f| f.min_silk_stroke)
    }

    fn check(&self, ctx: &LintContext<'_>) -> Vec<Diagnostic> {
        let Some(fab) = profile(ctx) else {
            return Vec::new();
        };
        let model = ctx.circuit.model();
        let texts = match fab.min_silk_height {
            Some(min) => fold(fab, "text", min, short_texts(model, min)),
            None => {
                (!model.silkscreen_texts.is_empty()).then(|| missing(&META, fab, "min_silk_height"))
            }
        };
        let lines = match fab.min_silk_stroke {
            Some(min) => fold(fab, "line", min, thin_paths(model, min)),
            None => {
                (!model.silkscreen_paths.is_empty()).then(|| missing(&META, fab, "min_silk_stroke"))
            }
        };
        texts.into_iter().chain(lines).collect()
    }
}
