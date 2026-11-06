use super::launch::studio_args;

#[test]
fn studio_args_add_share_only_when_pairing_is_allowed() {
    assert_eq!(
        studio_args(3020, false),
        ["studio", "--desktop", "-p", "3020"]
    );
    assert_eq!(
        studio_args(4000, true),
        ["studio", "--desktop", "-p", "4000", "--share"]
    );
}
