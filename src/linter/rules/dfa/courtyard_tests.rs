use super::Courtyard;

fn shape(points: &[(f64, f64)]) -> Courtyard {
    Courtyard {
        part: String::new(),
        layer: "top".into(),
        points: points.to_vec(),
    }
}

fn square(x: f64, y: f64, half: f64) -> Courtyard {
    shape(&[
        (x - half, y - half),
        (x + half, y - half),
        (x + half, y + half),
        (x - half, y + half),
    ])
}

fn notched() -> Courtyard {
    shape(&[
        (0.0, 0.0),
        (4.0, 0.0),
        (4.0, 4.0),
        (2.0, 4.0),
        (2.0, 2.0),
        (0.0, 2.0),
    ])
}

#[test]
fn a_part_in_the_notch_of_a_concave_outline_does_not_overlap() {
    assert!(!notched().overlaps(&square(1.0, 3.0, 0.4)));
    assert!(!square(1.0, 3.0, 0.4).overlaps(&notched()));
}

#[test]
fn crossing_contained_and_identical_courtyards_overlap() {
    assert!(notched().overlaps(&square(2.0, 2.0, 0.4)));
    assert!(notched().overlaps(&square(3.0, 1.0, 0.2)));
    assert!(square(0.0, 0.0, 1.0).overlaps(&square(0.0, 0.0, 1.0)));
}

#[test]
fn courtyards_that_only_touch_do_not_overlap() {
    assert!(!square(0.0, 0.0, 1.0).overlaps(&square(2.0, 0.0, 1.0)));
}
