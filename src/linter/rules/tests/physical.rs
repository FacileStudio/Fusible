//! Fixtures for the physics and fab rules: routed traces on named nets plus a manifest.

use std::path::Path;

use crate::config::{DesignRules, FusibleConfig};
use crate::linter::{Diagnostic, LintContext, Linter, Rule};
use crate::netlist::Netlist;
use crate::parser::parse;

/// One routed trace: net, layer, width, start and end in mm.
pub(super) type Trace<'a> = (&'a str, &'a str, f64, (f64, f64), (f64, f64));

/// Circuit JSON with a `source_net` per name and one `pcb_trace` per entry of `traces`.
pub(super) fn board(extra: &str, nets: &[&str], traces: &[Trace<'_>]) -> String {
    let mut items: Vec<String> = nets
        .iter()
        .map(|n| format!(r#"{{"type":"source_net","source_net_id":"n_{n}","name":"{n}"}}"#))
        .collect();
    for (i, (net, layer, width, a, b)) in traces.iter().enumerate() {
        items.push(format!(
            r#"{{"type":"source_trace","source_trace_id":"st{i}","connected_source_net_ids":["n_{net}"]}}"#
        ));
        items.push(format!(
            r#"{{"type":"pcb_trace","pcb_trace_id":"pt{i}","source_trace_id":"st{i}","route":[
            {{"route_type":"wire","x":{},"y":{},"width":{width},"layer":"{layer}"}},
            {{"route_type":"wire","x":{},"y":{},"width":{width},"layer":"{layer}"}}]}}"#,
            a.0, a.1, b.0, b.1
        ));
    }
    if !extra.is_empty() {
        items.push(extra.to_string());
    }
    format!("[{}]", items.join(","))
}

/// Runs `rule` alone on `json` with the design rules written in `manifest`.
pub(super) fn lint_with(rule: impl Rule + 'static, json: &str, manifest: &str) -> Vec<Diagnostic> {
    let circuit = parse(json, Path::new("circuit.json")).expect("fixture must parse");
    let netlist = Netlist::from_circuit(&circuit);
    let config = FusibleConfig::from_str(manifest).expect("manifest must parse");
    let design = DesignRules::from_config(&config);
    let ctx = LintContext::new(&circuit, &netlist).with_design(Some(&design));
    Linter::new().with_rule(rule).run_with_context(&ctx)
}
