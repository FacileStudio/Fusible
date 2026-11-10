//! Step 3 — rules engine (the linter core).
//!
//! The linter is a list of independent passes: each [`Rule`] inspects the
//! whole AST and returns its violations. Adding an electrical rule
//! (decoupling cap distance, pull-ups, voltage compatibility) = one new type
//! + one line in `Linter::default()`; nothing else in the pipeline moves.
//!
//! **Fail-closed contract**: a rule may only skip an element when another
//! registered rule is responsible for flagging it — see how
//! `UnconnectedNetRule` leans on `MalformedElementRule`. Silence is never
//! the default of this engine.
//!
//! Diagnostics and their miette rendering live in the `diagnostics`
//! submodule: only the engine stays here.
mod context;
mod cross;
mod diagnostics;
mod info;
mod rule;
mod rules;
#[cfg(test)]
mod tests;

pub use context::LintContext;
pub use cross::{cross_world_ids, CROSS_WORLD};
pub use diagnostics::{Diagnostic, Fix, FuseBlown, Location, Severity, SourceRef};
pub use info::{catalog, RuleInfo};
pub use rule::{Category, Rule, RuleMeta};
pub use rules::roles;
pub use rules::{
    AcidTrapRule, AnnularRingRule, BomIncompleteRule, ComponentEdgeClearanceRule,
    CourtyardClearanceRule, DrillToCopperRule, EdgeClearanceRule, FiducialsRule, HoleSpacingRule,
    MaskSliverRule, SilkOverPadRule, SilkTextSizeRule, ViaAspectRatioRule,
};
pub use rules::{
    BringUpRule, DanglingReferenceRule, DecouplingCapacitorRule, DeratingRule, FabProfileRule,
    GhostNetRule, LedSeriesResistorRule, MalformedElementRule, MissingPullupRule, NcConnectedRule,
    PinConflictRule, PinMuxRule, PinUnconnectedRule, PowerUndrivenRule, QfnFootprintSpanRule,
    RailBudgetRule, SchematicScaleRule, ShortedNetsRule, TraceClearanceRule, TraceCurrentRule,
    UnconnectedNetRule, UnroutedRule, UpstreamRule, VoltageClearanceRule, VoltageDomainRule,
};

/// The fuse: runs every pass and aggregates their violations.
pub struct Linter {
    rules: Vec<Box<dyn Rule>>,
}

impl Linter {
    /// An empty linter — rules are added explicitly with [`Self::with_rule`].
    /// Tests use this to exercise one rule in isolation.
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    /// Appends a rule. Registration order is display order: `run` is
    /// deterministic, so diagnostics never shuffle between runs.
    pub fn with_rule(mut self, rule: impl Rule + 'static) -> Self {
        self.rules.push(Box::new(rule));
        self
    }

    /// Runs every pass in registration order and concatenates their
    /// diagnostics, automatically constructing the lowered netlist. Test-only:
    /// commands build their own context with [`Self::run_with_context`].
    #[cfg(test)]
    pub fn run(&self, circuit: &crate::parser::Circuit) -> Vec<Diagnostic> {
        let netlist = crate::netlist::Netlist::from_circuit(circuit);
        let ctx = LintContext::new(circuit, &netlist);
        self.run_with_context(&ctx)
    }

    /// Runs every pass against a pre-built lint context.
    pub fn run_with_context(&self, ctx: &LintContext<'_>) -> Vec<Diagnostic> {
        self.rules
            .iter()
            .flat_map(|rule| {
                let severity = rule.severity();
                rule.check(ctx).into_iter().map(move |d| Diagnostic {
                    severity: d.severity.max(severity),
                    ..d
                })
            })
            .collect()
    }
}

impl Default for Linter {
    fn default() -> Self {
        Self::new()
            .with_rule(UnconnectedNetRule)
            .with_rule(MalformedElementRule)
            .with_rule(GhostNetRule)
            .with_rule(DanglingReferenceRule)
            .with_rule(ShortedNetsRule)
            .with_rule(PinMuxRule)
            .with_rule(DecouplingCapacitorRule)
            .with_rule(TraceClearanceRule)
            .with_rule(SchematicScaleRule)
            .with_rule(UnroutedRule)
            .with_rule(UpstreamRule)
            .with_rule(QfnFootprintSpanRule)
            .with_rule(TraceCurrentRule)
            .with_rule(VoltageClearanceRule)
            .with_rule(FabProfileRule)
            .with_fab_rules()
            .with_electrical_rules()
    }
}

impl Linter {
    /// Appends the fabrication and assembly rules.
    fn with_fab_rules(self) -> Self {
        self.with_rule(AnnularRingRule)
            .with_rule(DrillToCopperRule)
            .with_rule(HoleSpacingRule)
            .with_rule(ViaAspectRatioRule)
            .with_rule(EdgeClearanceRule)
            .with_rule(SilkOverPadRule)
            .with_rule(SilkTextSizeRule)
            .with_rule(MaskSliverRule)
            .with_rule(AcidTrapRule)
            .with_rule(CourtyardClearanceRule)
            .with_rule(ComponentEdgeClearanceRule)
            .with_rule(FiducialsRule)
            .with_rule(BomIncompleteRule)
    }

    /// Appends the electrical rules driven by pin roles.
    fn with_electrical_rules(self) -> Self {
        self.with_rule(PinConflictRule)
            .with_rule(PowerUndrivenRule)
            .with_rule(PinUnconnectedRule)
            .with_rule(NcConnectedRule)
            .with_rule(MissingPullupRule)
            .with_rule(VoltageDomainRule)
            .with_rule(RailBudgetRule)
            .with_rule(DeratingRule)
            .with_rule(LedSeriesResistorRule)
            .with_rule(BringUpRule)
    }
}
