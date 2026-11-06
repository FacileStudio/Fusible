//! The limits of the mechanical rules, by process, with the guideline each one comes from.

/// Enclosure design guide for 3D printing (walls, bosses, vents, standoffs).
pub(super) const DFAM: &str = "https://www.dfamacademy.com/guides/enclosure-design-3d-printing";

/// Enclosure design guide with the 0.5 mm gap around internal parts and the port margin.
pub(super) const HUBS: &str =
    "https://www.hubs.com/knowledge-base/enclosure-design-3d-printing-step-step-guide/";

/// Injection molding part design guide (wall, boss and draft limits).
pub(super) const RTP: &str =
    "https://www.rtpcompany.com/wp-content/uploads/Part-Design-Guidelines-Brochure.pdf";

/// Appended to every message of the mechanical rules.
pub(super) const DISCLAIMER: &str = "Design guideline, not a certification.";

/// Added to the check of an insert size, because the table is not from a datasheet.
pub(super) const INSERT_NOTE: &str = "typical insert size, check your insert's datasheet";

/// Thickest injection molded wall before it sinks, in mm.
pub(super) const INJECTION_WALL_MAX: f64 = 5.0;

/// Injection molded boss outer diameter, at most this many times its hole.
pub(super) const INJECTION_BOSS_RATIO: f64 = 2.5;

/// Boss outer diameter, at least this many times its hole.
pub(super) const BOSS_RATIO: f64 = 2.0;

/// Smallest draft angle a molded part declares, in degrees.
pub(super) const MIN_DRAFT_DEG: f64 = 0.5;

/// How much taller than the lowest part under the board a standoff stays, in mm.
pub(super) const STANDOFF_MARGIN: f64 = 0.5;

/// Thinnest wall of a process, in mm.
pub(super) fn wall_min(process: &str) -> f64 {
    match process {
        "fdm" => 2.0,
        "sla" | "mjf" => 1.5,
        "injection" | "cnc" => 1.0,
        _ => 2.0,
    }
}

/// Smallest margin around a port cutout, per side, in mm. Molded and machined parts have no limit.
pub(super) fn port_margin(process: &str) -> Option<f64> {
    match process {
        "fdm" => Some(1.0),
        "sla" | "mjf" => Some(0.5),
        _ => None,
    }
}

/// Narrowest vent slot, in mm. Molded and machined parts have no limit.
pub(super) fn vent_min(process: &str) -> Option<f64> {
    match process {
        "fdm" => Some(1.0),
        "sla" | "mjf" => Some(0.8),
        _ => None,
    }
}

/// Smallest gap between the part and the components of the board, in mm.
pub fn clearance_gap(process: &str) -> f64 {
    if process == "fdm" {
        0.5
    } else {
        0.2
    }
}

/// Bore of a typical heat-set insert for a screw size.
pub(super) fn insert_bore(size: &str) -> Option<f64> {
    match size {
        "m2" => Some(3.2),
        "m2.5" => Some(3.6),
        "m3" => Some(4.0),
        _ => None,
    }
}

#[cfg(test)]
#[path = "mech_rules_wall_tests.rs"]
mod wall_tests;
