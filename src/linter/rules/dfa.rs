//! Design for assembly: what the assembler needs to place and buy every part.

mod bom_incomplete;
mod component_edge_clearance;
mod courtyard;
mod courtyard_clearance;
mod fiducials;

pub use bom_incomplete::BomIncompleteRule;
pub use component_edge_clearance::ComponentEdgeClearanceRule;
pub use courtyard_clearance::CourtyardClearanceRule;
pub use fiducials::FiducialsRule;
