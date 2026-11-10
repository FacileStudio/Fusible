//! Design for manufacturing: what the fab house can drill, plate, mask and print.
//!
//! Every rule here reads one limit of the manifest's `fab:` profile and stays
//! silent without one. A profile that does not publish the number disables
//! the rule with one `info` diagnostic. `acid_trap` reads no limit.

mod acid_trap;
mod annular_ring;
mod drill_to_copper;
mod drills;
mod edge_clearance;
mod hole_spacing;
mod index;
mod limit;
mod mask_sliver;
mod outline;
mod owners;
mod silk;
mod silk_over_pad;
mod silk_text_size;
mod via_aspect_ratio;

pub use acid_trap::AcidTrapRule;
pub use annular_ring::AnnularRingRule;
pub use drill_to_copper::DrillToCopperRule;
pub use edge_clearance::EdgeClearanceRule;
pub use hole_spacing::HoleSpacingRule;
pub use mask_sliver::MaskSliverRule;
pub use silk_over_pad::SilkOverPadRule;
pub use silk_text_size::SilkTextSizeRule;
pub use via_aspect_ratio::ViaAspectRatioRule;

pub(super) use index::Index;
pub(super) use limit::{at, declared, violation, Gate};
pub(super) use outline::{outline, Outline};
pub(super) use owners::Owners;
