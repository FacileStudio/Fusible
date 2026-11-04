use super::clock::rfc3339;

#[test]
fn rfc3339_formats_utc() {
    assert_eq!(rfc3339(0), "1970-01-01T00:00:00Z");
    assert_eq!(rfc3339(1_709_210_096), "2024-02-29T12:34:56Z");
}

#[test]
fn the_summary_lists_each_board_with_counts() {
    use super::summary::{counts, summary_json};
    use crate::commands::target::BoardTarget;
    let board = BoardTarget {
        name: Some("main".into()),
        entry: std::path::PathBuf::from("/p/main/c.json"),
        ..Default::default()
    };
    let state = crate::dev::state::State::new(&[board]);
    let json: serde_json::Value = serde_json::from_str(&summary_json(&state)).unwrap();
    assert_eq!(json["boards"][0]["name"], "main");
    assert_eq!(json["boards"][0]["fuse_blown"], true);
    assert!(json["last_run"].is_null());
    assert!(json["bodies"].is_array());
    let diags = serde_json::json!({"diagnostics":[{"severity":"error"},{"severity":"warning"},{"severity":"warning"}]});
    assert_eq!(counts(&diags, true), (1, 2));
    let bare = serde_json::json!({"diagnostics":[{"rule":"a"},{"rule":"b"}]});
    assert_eq!(counts(&bare, true), (2, 0));
    assert_eq!(counts(&bare, false), (0, 2));
}
