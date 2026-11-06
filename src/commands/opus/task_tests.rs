use super::clean_title;

#[test]
fn a_title_never_starts_with_a_dash() {
    assert_eq!(clean_title("--label x: boom"), "[fusible] --label x: boom");
    assert_eq!(clean_title("-h"), "[fusible] -h");
    assert_eq!(clean_title("main: ok"), "main: ok");
}
