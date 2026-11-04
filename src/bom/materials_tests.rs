use super::materials::{library, resolve, validate};
use super::stl::stl_mass;

#[test]
fn the_library_holds_the_eight_materials_with_their_processes() {
    assert_eq!(library().len(), 8);
    assert_eq!(library()["al6061"].processes, ["cnc"]);
    assert_eq!(library()["abs"].processes, ["fdm", "injection"]);
}

#[test]
fn a_material_must_exist_and_support_the_process() {
    assert!(validate(Some("petg"), Some("fdm")).is_ok());
    assert!(validate(None, Some("sla")).is_ok());
    assert!(validate(Some("unobtainium"), None).is_err());
    assert!(validate(Some("petg"), Some("cnc")).is_err());
    assert!(validate(Some("petg"), Some("laser")).is_err());
}

#[test]
fn a_body_without_a_process_takes_the_first_of_its_material() {
    assert_eq!(resolve(None, None), ("pla".into(), "fdm".into()));
    assert_eq!(
        resolve(Some("al6061"), None),
        ("al6061".into(), "cnc".into())
    );
    assert_eq!(resolve(Some("pa12"), Some("mjf")).1, "mjf");
}

#[test]
fn fdm_counts_the_effective_fill_and_other_processes_a_solid() {
    let path = std::env::temp_dir().join(format!("fusible-mass-{}.stl", std::process::id()));
    let mut text = String::from("solid c\n");
    let cube = [
        ([0, 0, 0], [10, 0, 0], [0, 10, 0]),
        ([0, 0, 0], [0, 10, 0], [0, 0, 10]),
        ([0, 0, 0], [0, 0, 10], [10, 0, 0]),
        ([10, 0, 0], [0, 0, 10], [0, 10, 0]),
    ];
    for (a, b, c) in cube {
        text.push_str("facet normal 0 0 0\nouter loop\n");
        for v in [a, b, c] {
            text.push_str(&format!("vertex {} {} {}\n", v[0], v[1], v[2]));
        }
        text.push_str("endloop\nendfacet\n");
    }
    text.push_str("endsolid c\n");
    std::fs::write(&path, text).unwrap();
    let al = &library()["al6061"];
    let solid = stl_mass(&path, al, "cnc").unwrap();
    let printed = stl_mass(&path, al, "fdm").unwrap();
    let _ = std::fs::remove_file(&path);
    assert!((solid - 0.45).abs() < 0.06, "{solid}");
    assert!(printed < solid);
}
