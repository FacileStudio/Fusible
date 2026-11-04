//! Mechanical rules on bosses, standoffs, ports and vents.

use super::mech_rules::{Finding, BOSS, PORT_CLEARANCE, STANDOFF_HEIGHT, VENT_WIDTH};
use super::mech_rules_data::*;
use super::part_features::{Below, Feature};
use crate::linter::Severity;

fn insert_findings(part: &str, size: &str, bore: Option<f64>) -> Option<Finding> {
    let want = insert_bore(size)?;
    let got = bore?;
    ((got - want).abs() > 1e-6).then(|| {
        let text = format!(
            "`{part}` has a {size} insert boss with a {got} mm bore; the table says {want} mm ({INSERT_NOTE})."
        );
        Finding::error(BOSS, text, "use the bore of your heat-set insert", DFAM)
    })
}

fn boss_shape(part: &str, process: &str, outer: f64, hole: f64) -> Vec<Finding> {
    let mut found = Vec::new();
    if outer < BOSS_RATIO * hole {
        let text = format!(
            "`{part}` has a boss of {outer} mm around a {hole} mm hole; use at least {} mm.",
            BOSS_RATIO * hole
        );
        found.push(Finding::new(
            BOSS,
            Severity::Warning,
            text,
            "make the boss twice the hole",
            DFAM,
        ));
    }
    if process == "injection" && outer > INJECTION_BOSS_RATIO * hole {
        let text = format!("`{part}` has a boss of {outer} mm around a {hole} mm hole; molded bosses stay under {} mm.", INJECTION_BOSS_RATIO * hole);
        found.push(Finding::error(
            BOSS,
            text,
            "slim the boss and add gussets to stiffen it",
            RTP,
        ));
    }
    found
}

/// Checks insert bores, boss diameters and, for molded parts, boss thickness.
pub(super) fn boss(part: &str, process: &str, features: &[Feature]) -> Vec<Finding> {
    let mut found = Vec::new();
    for feature in features {
        let Feature::Boss {
            insert,
            bore,
            pilot,
            outer,
            ..
        } = feature
        else {
            continue;
        };
        found.extend(
            insert
                .as_deref()
                .and_then(|s| insert_findings(part, s, *bore)),
        );
        if let Some(hole) = bore.or(*pilot) {
            found.extend(boss_shape(part, process, *outer, hole));
        }
    }
    found
}

/// Checks the margin around every port cutout.
pub(super) fn port_clearance(part: &str, process: &str, features: &[Feature]) -> Vec<Finding> {
    let Some(min) = port_margin(process) else {
        return Vec::new();
    };
    features
        .iter()
        .filter_map(|f| match f {
            Feature::Port { margin } if *margin < min => {
                let text = format!("`{part}` has a port with {margin} mm of margin per side; {process} needs {min} mm.");
                Some(Finding::error(PORT_CLEARANCE, text, "widen the cutout around the connector", HUBS))
            }
            _ => None,
        })
        .collect()
}

/// Checks the width of every vent slot.
pub(super) fn vent_width(part: &str, process: &str, features: &[Feature]) -> Vec<Finding> {
    let Some(min) = vent_min(process) else {
        return Vec::new();
    };
    features
        .iter()
        .filter_map(|f| match f {
            Feature::Vent { w } if *w < min => {
                let text =
                    format!("`{part}` has a {w} mm vent slot; {process} needs at least {min} mm.");
                Some(Finding::error(VENT_WIDTH, text, "widen the slot", DFAM))
            }
            _ => None,
        })
        .collect()
}

fn too_short(part: &str, h: f64, below: &Option<Below>) -> Option<Finding> {
    let under = below.as_ref()?;
    (h < under.depth + STANDOFF_MARGIN).then(|| {
        let text = format!(
            "`{part}`: {} sits {:.1} mm under the board; standoffs are {h} mm.",
            under.component, under.depth
        );
        let help = "raise the standoff to the deepest part under the board plus 0.5 mm";
        Finding::error(STANDOFF_HEIGHT, text, help, DFAM)
    })
}

/// Checks that standoffs and bosses clear the parts under the board.
pub(super) fn standoff_height(part: &str, features: &[Feature]) -> Vec<Finding> {
    features
        .iter()
        .filter_map(|f| match f {
            Feature::Standoff { h, below } => too_short(part, *h, below),
            Feature::Boss {
                h: Some(h), below, ..
            } => too_short(part, *h, below),
            _ => None,
        })
        .collect()
}
