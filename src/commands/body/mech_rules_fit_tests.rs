use super::mech_rules::*;
use super::part_features::{Below, Feature};

fn ids(found: &[Finding]) -> Vec<&'static str> {
    found.iter().map(|f| f.rule).collect()
}

fn wall(thickness: f64) -> Feature {
    Feature::Wall {
        thickness,
        part: None,
    }
}

#[test]
fn a_port_needs_its_margin_by_process() {
    let port = |margin| [Feature::Port { margin }];
    assert_eq!(ids(&check("p", "fdm", &port(0.5))), [PORT_CLEARANCE]);
    assert!(check("p", "fdm", &port(1.0)).is_empty());
    assert!(check("p", "sla", &port(0.5)).is_empty());
    assert_eq!(ids(&check("p", "mjf", &port(0.2))), [PORT_CLEARANCE]);
}

#[test]
fn a_vent_slot_needs_its_width_by_process() {
    let vent = |w| [Feature::Vent { w }];
    assert_eq!(ids(&check("p", "fdm", &vent(0.6))), [VENT_WIDTH]);
    assert!(check("p", "fdm", &vent(1.0)).is_empty());
    assert!(check("p", "sla", &vent(0.8)).is_empty());
    assert_eq!(ids(&check("p", "sla", &vent(0.6))), [VENT_WIDTH]);
}

#[test]
fn a_short_standoff_names_the_part_under_the_board() {
    let below = Some(Below {
        component: "U3".into(),
        depth: 2.1,
    });
    let short = Feature::Standoff {
        h: 1.5,
        below: below.clone(),
    };
    let found = check("p", "fdm", &[short]);
    assert_eq!(ids(&found), [STANDOFF_HEIGHT]);
    assert!(found[0]
        .message
        .contains("U3 sits 2.1 mm under the board; standoffs are 1.5 mm"));
    assert!(check("p", "fdm", &[Feature::Standoff { h: 2.6, below }]).is_empty());
    assert!(check(
        "p",
        "fdm",
        &[Feature::Standoff {
            h: 1.0,
            below: None
        }]
    )
    .is_empty());
}

#[test]
fn a_part_inside_the_gap_of_a_component_blows() {
    let hit = Feature::Interference {
        component: "J1".into(),
        volume: 4.0,
        gap: 0.5,
    };
    assert_eq!(ids(&check("p", "fdm", &[hit])), [COMPONENT_CLEARANCE]);
    assert_eq!(
        ids(&check("p", "fdm", &[Feature::Clearance { gap: 0.3 }])),
        [COMPONENT_CLEARANCE]
    );
    assert!(check("p", "sla", &[Feature::Clearance { gap: 0.3 }]).is_empty());
}

#[test]
fn every_message_ends_with_the_disclaimer_and_names_its_source() {
    let found = check("p", "fdm", &[wall(1.0), Feature::Vent { w: 0.5 }]);
    for finding in &found {
        assert!(finding
            .message
            .ends_with("Design guideline, not a certification."));
        assert!(finding.source_url.starts_with("https://"));
        assert_eq!(
            finding.diagnostic().source_url.as_deref(),
            Some(finding.source_url)
        );
    }
}
