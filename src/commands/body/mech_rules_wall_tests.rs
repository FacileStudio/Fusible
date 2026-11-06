#[test]
fn an_unknown_process_gets_the_strictest_wall_minimum() {
    assert_eq!(super::wall_min("bogus"), 2.0);
    assert_eq!(super::wall_min("cnc"), 1.0);
}
