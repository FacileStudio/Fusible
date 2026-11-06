//! The rule set: every electrical/structural check the fuse can blow on.
//!
//! Phase 1 ships three rules that together make the pipeline **fail-closed**:
//!
//! * `UnconnectedNetRule` — a net with fewer than two endpoints is floating;
//! * `MalformedElementRule` — a recognized element without its identifier
//!   cannot be verified, so it is a violation, not a shrug;
//! * `GhostNetRule` — a `net_id` referenced by a port or pad but defined by
//!   no `source_net` describes copper that does not exist.
//!
//! The division of labor matters: `UnconnectedNetRule` skips nets it cannot
//! identify, *because* `MalformedElementRule` reports them. A rule may only
//! stay silent when another registered rule speaks for what it skipped.
//!
//! One rule per file — filet's functions-per-file budget counts `impl`
//! methods, so a hub module keeps every rule comfortably under the limit.

mod autoroute;
mod autoroute_scan;
mod bypass_cap;
pub(super) mod bypass_distance;
mod copper;
mod copper_drill;
mod dangling_reference;
mod fab_profile;
mod geom2d;
mod ghost_net;
mod ipc2152;
mod ipc2221b;
mod malformed_element;
mod pad_geometry;
mod physics;
mod pin_mux;
mod qfn_footprint;
mod schematic_scale;
mod shorted_nets;
#[cfg(test)]
mod tests;
mod trace_clearance;
mod trace_current;
mod unconnected_net;
mod voltage_clearance;

pub use autoroute::AutorouteRule;
pub use bypass_cap::BypassCapRule;
pub use dangling_reference::DanglingReferenceRule;
pub use fab_profile::FabProfileRule;
pub use ghost_net::GhostNetRule;
pub use malformed_element::MalformedElementRule;
pub use pin_mux::PinMuxRule;
pub use qfn_footprint::QfnFootprintSpanRule;
pub use schematic_scale::SchematicScaleRule;
pub use shorted_nets::ShortedNetsRule;
pub use trace_clearance::TraceClearanceRule;
pub use trace_current::TraceCurrentRule;
pub use unconnected_net::UnconnectedNetRule;
pub use voltage_clearance::VoltageClearanceRule;
