//! Diagnostics of the `trace_current` rule.

use super::{Hottest, Load, SOURCE, STANDARD};
use crate::linter::rules::ipc2152::min_width;
use crate::linter::rules::physics::{is_external, PRECHECK};
use crate::linter::{Diagnostic, Severity};

pub(super) fn diagnostic(message: String, severity: Severity) -> Diagnostic {
    Diagnostic {
        rule: "trace_current".to_string(),
        message,
        severity,
        standard: Some(STANDARD.to_string()),
        source_url: Some(SOURCE.to_string()),
        ..Default::default()
    }
}

fn fix(load: &Load<'_>, hottest: &Hottest<'_>) -> String {
    let internal = !is_external(&hottest.segment.layer);
    match min_width(load.amps, load.max_rise, internal, hottest.oz) {
        Some(w) => format!("widen {} to at least {w:.2} mm", load.net),
        None => format!(
            "no width up to 20 mm carries {} A; split the net over more copper",
            load.amps
        ),
    }
}

pub(super) fn violation(load: &Load<'_>, hottest: &Hottest<'_>, all: &[Hottest<'_>]) -> Diagnostic {
    let Hottest { segment, rise, .. } = hottest;
    let narrowest = all.iter().all(|h| h.segment.width >= segment.width);
    let which = if narrowest { "narrowest" } else { "hottest" };
    let mut found = diagnostic(
        format!(
            "{} carries {:.1} A; its {which} trace ({} mm, {}) heats about {:.0} \u{b0}C over ambient, \
             above the {} \u{b0}C limit",
            load.net, load.amps, segment.width, segment.layer, rise, load.max_rise
        ),
        Severity::Error,
    );
    found.help = Some(format!(
        "IPC-2152 estimate (Brooks and Adam fit). {PRECHECK}"
    ));
    found.fix = Some(fix(load, hottest));
    let over = all.iter().filter(|h| h.rise > load.max_rise);
    let mut ids: Vec<String> = over.map(|h| h.segment.trace_id.clone()).collect();
    ids.dedup();
    found.elements = ids;
    found
}
