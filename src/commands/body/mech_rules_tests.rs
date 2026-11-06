use super::mech_rules::*;
use super::part_features::{Feature, PartReport};
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
fn a_report_is_read_from_the_last_json_line_of_the_script_output() {
    let features = r#"[{"kind":"vent","w":2.0,"at":[0,0,0]},{"kind":"later"}]"#;
    let out = format!("noise\n{{\"features\":{features},\"bbox\":[1,2,3]}}\n");
    let report = PartReport::from_output(&out).unwrap();
    assert_eq!(report.features, [Feature::Vent { w: 2.0 }, Feature::Other]);
    assert_eq!(report.bbox, Some([1.0, 2.0, 3.0]));
    assert!(PartReport::from_output("no json").is_none());
}
