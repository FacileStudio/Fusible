//! The rule id of each tscircuit `*_error` and `*_warning` type.
//!
//! Types are the `type` literals of tscircuit/circuit-json 0.0.548
//! (`src/pcb/*_error.ts`, `src/source/*_warning.ts`). An error type missing
//! from [`KINDS`] still blows the fuse as [`UPSTREAM_ERROR`].

use crate::linter::{Category, RuleMeta, Severity};

const fn upstream(id: &'static str, summary: &'static str, severity: Severity) -> RuleMeta {
    RuleMeta {
        id,
        category: Category::Upstream,
        summary,
        standard: Some("tscircuit"),
        source_url: None,
        default_severity: severity,
    }
}

const E: Severity = Severity::Error;

/// Any tscircuit `*_error` no other id covers. Fail closed: it blows the fuse.
pub(super) const UPSTREAM_ERROR: RuleMeta = upstream(
    "upstream_error",
    "tscircuit reported an error no other rule names.",
    E,
);
/// Any tscircuit `*_warning` no other id covers.
pub(super) const UPSTREAM_WARNING: RuleMeta = upstream(
    "upstream_warning",
    "tscircuit reported a warning no other rule names.",
    Severity::Warning,
);
const FOOTPRINT_OVERLAP: RuleMeta = upstream(
    "footprint_overlap",
    "tscircuit found two footprints overlapping.",
    E,
);
const COURTYARD_OVERLAP: RuleMeta = upstream(
    "courtyard_overlap",
    "tscircuit found two part courtyards overlapping.",
    E,
);
const PLACEMENT: RuleMeta = upstream("placement", "tscircuit could not place a part legally.", E);
const OFF_BOARD: RuleMeta = upstream(
    "off_board",
    "tscircuit found a part outside the board outline.",
    E,
);
const TRACE_ERROR: RuleMeta = upstream("trace_error", "tscircuit found a faulty routed trace.", E);
const TRACE_MISSING: RuleMeta = upstream(
    "trace_missing",
    "tscircuit found a connection with no PCB trace.",
    E,
);
const PORT_NOT_CONNECTED: RuleMeta = upstream(
    "port_not_connected",
    "tscircuit found ports of one trace on different nets.",
    E,
);
const VIA_CLEARANCE: RuleMeta = upstream(
    "via_clearance",
    "tscircuit found two vias closer than allowed.",
    E,
);
const PAD_TRACE_CLEARANCE: RuleMeta = upstream(
    "pad_trace_clearance",
    "tscircuit found a trace too close to a pad.",
    E,
);
const VIA_TRACE_CLEARANCE: RuleMeta = upstream(
    "via_trace_clearance",
    "tscircuit found a trace too close to a via.",
    E,
);
const PAD_PAD_CLEARANCE: RuleMeta = upstream(
    "pad_pad_clearance",
    "tscircuit found two pads closer than allowed.",
    E,
);
const AUTOROUTING: RuleMeta = upstream(
    "autorouting",
    "tscircuit's autorouter failed or was skipped.",
    E,
);
const CONNECTOR_ACCESSIBILITY: RuleMeta = upstream(
    "connector_accessibility",
    "tscircuit found a connector that cannot be reached.",
    E,
);
const UNNAMED_TRACE: RuleMeta = upstream(
    "unnamed_trace",
    "tscircuit found a trace without a name; informational.",
    Severity::Info,
);

/// Upstream type to rule id. Every `*_error` row maps to an error-level id.
pub(in crate::linter::rules) const KINDS: [(&str, &RuleMeta); 14] = [
    ("pcb_footprint_overlap_error", &FOOTPRINT_OVERLAP),
    ("pcb_courtyard_overlap_error", &COURTYARD_OVERLAP),
    ("pcb_placement_error", &PLACEMENT),
    ("pcb_component_outside_board_error", &OFF_BOARD),
    ("pcb_trace_error", &TRACE_ERROR),
    ("pcb_trace_missing_error", &TRACE_MISSING),
    ("pcb_port_not_connected_error", &PORT_NOT_CONNECTED),
    ("pcb_via_clearance_error", &VIA_CLEARANCE),
    ("pcb_pad_trace_clearance_error", &PAD_TRACE_CLEARANCE),
    ("pcb_via_trace_clearance_error", &VIA_TRACE_CLEARANCE),
    ("pcb_pad_pad_clearance_error", &PAD_PAD_CLEARANCE),
    ("pcb_autorouting_error", &AUTOROUTING),
    ("pcb_accessibility_error", &CONNECTOR_ACCESSIBILITY),
    ("source_unnamed_trace_warning", &UNNAMED_TRACE),
];

/// Every id besides [`UPSTREAM_ERROR`] that the upstream rule reports under.
pub(super) const EMITS: [&RuleMeta; 15] = [
    &UPSTREAM_WARNING,
    &FOOTPRINT_OVERLAP,
    &COURTYARD_OVERLAP,
    &PLACEMENT,
    &OFF_BOARD,
    &TRACE_ERROR,
    &TRACE_MISSING,
    &PORT_NOT_CONNECTED,
    &VIA_CLEARANCE,
    &PAD_TRACE_CLEARANCE,
    &VIA_TRACE_CLEARANCE,
    &PAD_PAD_CLEARANCE,
    &AUTOROUTING,
    &CONNECTOR_ACCESSIBILITY,
    &UNNAMED_TRACE,
];

/// The rule id an upstream issue of type `kind` reports under.
pub(super) fn rule_for(kind: &str, is_error: bool) -> &'static RuleMeta {
    let mapped = KINDS.iter().find(|(name, _)| *name == kind);
    match (mapped, is_error) {
        (Some((_, meta)), _) => meta,
        (None, true) => &UPSTREAM_ERROR,
        (None, false) => &UPSTREAM_WARNING,
    }
}
