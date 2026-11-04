use super::mech_rules::*;
use super::part_features::{Below, Feature, PartReport};
use crate::linter::Severity;

fn ids(found: &[Finding]) -> Vec<&'static str> {
    found.iter().map(|f| f.rule).collect()
}

fn wall(thickness: f64) -> Feature {
    Feature::Wall {
        thickness,
        part: None,
    }
}

fn boss(insert: Option<&str>, bore: f64, outer: f64) -> Feature {
    Feature::Boss {
        insert: insert.map(str::to_string),
        bore: Some(bore),
        pilot: None,
        outer,
        h: None,
        below: None,
    }
}

#[test]
fn a_thin_fdm_wall_blows_and_a_thick_one_holds() {
    assert_eq!(ids(&check("p", "fdm", &[wall(1.2)])), [WALL_THICKNESS]);
    assert!(check("p", "fdm", &[wall(2.0)]).is_empty());
    assert!(check("p", "sla", &[wall(1.5)]).is_empty());
    assert_eq!(ids(&check("p", "mjf", &[wall(1.2)])), [WALL_THICKNESS]);
    assert!(check("p", "cnc", &[wall(1.0)]).is_empty());
}

#[test]
fn injection_walls_stay_between_one_and_five_millimetres_and_need_draft() {
    let draft = Feature::Draft { deg: 1.0 };
    assert!(check("p", "injection", &[wall(3.0), draft.clone()]).is_empty());
    assert_eq!(
        ids(&check("p", "injection", &[wall(6.0), draft.clone()])),
        [WALL_THICKNESS]
    );
    assert_eq!(
        ids(&check("p", "injection", &[wall(0.8), draft])),
        [WALL_THICKNESS]
    );
    let thin = Feature::Draft { deg: 0.2 };
    assert_eq!(ids(&check("p", "injection", &[wall(3.0), thin])), [DRAFT]);
    assert_eq!(ids(&check("p", "injection", &[wall(3.0)])), [DRAFT]);
    assert!(check("p", "fdm", &[wall(3.0)]).is_empty());
}

#[test]
fn an_insert_boss_must_match_the_table_and_be_twice_the_bore() {
    assert!(check("p", "fdm", &[boss(Some("m3"), 4.0, 8.0)]).is_empty());
    let wrong = check("p", "fdm", &[boss(Some("m3"), 3.0, 8.0)]);
    assert_eq!(ids(&wrong), [BOSS]);
    assert!(wrong[0].message.contains("check your insert's datasheet"));
    let thin = check("p", "fdm", &[boss(Some("m3"), 4.0, 6.0)]);
    assert_eq!(ids(&thin), [BOSS]);
    assert_eq!(thin[0].severity, Severity::Warning);
}

#[test]
fn a_molded_boss_is_at_most_two_and_a_half_times_its_hole() {
    let draft = Feature::Draft { deg: 1.0 };
    let found = check("p", "injection", &[boss(None, 2.5, 9.0), draft]);
    assert_eq!(ids(&found), [BOSS]);
    assert_eq!(found[0].severity, Severity::Error);
    assert!(found[0].help.contains("gussets"));
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
        assert!(finding
            .diagnostic()
            .help
            .unwrap()
            .contains(finding.source_url));
    }
}

#[test]
fn a_report_is_read_from_the_last_json_line_of_the_script_output() {
    let out = "noise\n{\"features\":[{\"kind\":\"vent\",\"w\":2.0,\"at\":[0,0,0]},{\"kind\":\"later\"}],\"bbox\":[1,2,3]}\n";
    let report = PartReport::from_output(out).unwrap();
    assert_eq!(report.features, [Feature::Vent { w: 2.0 }, Feature::Other]);
    assert_eq!(report.bbox, Some([1.0, 2.0, 3.0]));
    assert!(PartReport::from_output("no json").is_none());
}
