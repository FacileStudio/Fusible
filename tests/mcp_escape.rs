//! `edit_board` refuses files outside the project root.

mod common;
mod fake_support;

use common::TestDir;
use fake_support::rpc::{call, is_error, session, structured};
use fake_support::{git_commit_all, repo, BLOWN, INTACT};
use serde_json::json;

fn blown_text() -> String {
    std::fs::read_to_string(repo(BLOWN)).unwrap()
}

#[test]
fn a_board_file_outside_the_root_is_refused() {
    let parent = TestDir::new("escape");
    let root = parent.path.join("proj");
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(parent.path.join("outside.json"), "[]").unwrap();
    std::fs::copy(repo(INTACT), root.join("circuit.json")).unwrap();
    let manifest = "name: escape\nboards:\n  main: {entry: circuit.json}\n  escape: {entry: ../outside.json}\n";
    std::fs::write(root.join("fusible.yml"), manifest).unwrap();
    git_commit_all(&root);

    let args = json!({"board": "escape", "content": blown_text(), "dry_run": false});
    let replies = session(&root, &[call(1, "edit_board", args)]);

    assert!(is_error(&replies[0]));
    assert!(structured(&replies[0])["error"]
        .as_str()
        .unwrap()
        .contains("outside the project root"));
    assert_eq!(
        std::fs::read_to_string(parent.path.join("outside.json")).unwrap(),
        "[]"
    );
}

#[test]
fn a_symlink_that_escapes_the_root_is_refused() {
    let parent = TestDir::new("symlink");
    let root = parent.path.join("proj");
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(parent.path.join("outside.json"), "[]").unwrap();
    std::os::unix::fs::symlink(parent.path.join("outside.json"), root.join("linked.json")).unwrap();
    std::fs::write(
        root.join("fusible.yml"),
        "name: link\nboards:\n  linked: {entry: linked.json}\n",
    )
    .unwrap();
    git_commit_all(&root);

    let args = json!({"board": "linked", "content": blown_text(), "dry_run": false});
    let replies = session(&root, &[call(1, "edit_board", args)]);

    assert!(is_error(&replies[0]));
    assert_eq!(
        std::fs::read_to_string(parent.path.join("outside.json")).unwrap(),
        "[]"
    );
}
