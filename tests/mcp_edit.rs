//! `edit_board` write safety: dry run, dirty files, escapes and patches.

mod common;
mod fake_support;

use std::path::Path;
use std::process::Command;

use common::TestDir;
use fake_support::rpc::{call, is_error, session, structured};
use fake_support::{git_commit_all, project, repo, BLOWN, INTACT};
use serde_json::json;

fn blown_text() -> String {
    std::fs::read_to_string(repo(BLOWN)).unwrap()
}

fn git_output(dir: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .current_dir(dir)
        .args(args)
        .output()
        .unwrap();
    String::from_utf8(out.stdout).unwrap()
}

fn committed_project() -> TestDir {
    let dir = project(INTACT);
    git_commit_all(&dir.path);
    dir
}

#[test]
fn a_dry_run_leaves_the_file_bytes_identical() {
    let dir = committed_project();
    let file = dir.path.join("circuit.json");
    let before = std::fs::read(&file).unwrap();

    let replies = session(
        &dir.path,
        &[call(
            1,
            "edit_board",
            json!({"board": "main", "content": blown_text()}),
        )],
    );

    let result = structured(&replies[0]);
    assert_eq!(result["applied"], false);
    assert_eq!(result["dry_run"], true);
    assert_eq!(result["fuse"], "blown");
    assert!(result["diff"].as_str().unwrap().contains("I2C_SDA"));
    assert_eq!(std::fs::read(&file).unwrap(), before);
}

#[test]
fn a_real_edit_keeps_the_change_and_commits_nothing() {
    let dir = committed_project();
    let args = json!({"board": "main", "content": blown_text(), "dry_run": false});

    let replies = session(
        &dir.path,
        &[call(1, "edit_board", args), call(2, "check", json!({}))],
    );

    assert_eq!(structured(&replies[0])["applied"], true);
    assert_eq!(structured(&replies[1])["fuse"], "blown");
    assert_eq!(
        std::fs::read_to_string(dir.path.join("circuit.json")).unwrap(),
        blown_text()
    );
    assert_eq!(
        git_output(&dir.path, &["rev-list", "--count", "HEAD"]).trim(),
        "1"
    );
}

#[test]
fn a_second_edit_is_allowed_because_this_session_wrote_the_first() {
    let dir = committed_project();
    let first = json!({"board": "main", "content": blown_text(), "dry_run": false});
    let second = json!({"board": "main", "content": std::fs::read_to_string(repo(INTACT)).unwrap(), "dry_run": false});

    let replies = session(
        &dir.path,
        &[call(1, "edit_board", first), call(2, "edit_board", second)],
    );

    assert_eq!(structured(&replies[1])["applied"], true);
    assert_eq!(structured(&replies[1])["fuse"], "intact");
}

#[test]
fn changes_made_outside_the_session_are_refused() {
    let dir = committed_project();
    let file = dir.path.join("circuit.json");
    let mine = format!("{}\n", std::fs::read_to_string(&file).unwrap());
    std::fs::write(&file, &mine).unwrap();

    let replies = session(
        &dir.path,
        &[call(
            1,
            "edit_board",
            json!({"board": "main", "content": blown_text()}),
        )],
    );

    assert!(is_error(&replies[0]));
    assert!(structured(&replies[0])["error"]
        .as_str()
        .unwrap()
        .contains("did not make"));
    assert_eq!(std::fs::read_to_string(&file).unwrap(), mine);
}

#[test]
fn exactly_one_of_patch_or_content_is_required() {
    let dir = committed_project();
    let both = json!({"board": "main", "content": "x", "patch": "y"});
    let neither = json!({"board": "main"});

    let replies = session(
        &dir.path,
        &[call(1, "edit_board", both), call(2, "edit_board", neither)],
    );

    assert!(is_error(&replies[0]) && is_error(&replies[1]));
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

#[cfg(unix)]
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

#[test]
fn a_patch_applies_and_a_patch_for_another_file_is_refused() {
    let dir = committed_project();
    let file = dir.path.join("circuit.json");
    std::fs::write(&file, blown_text()).unwrap();
    let patch = git_output(&dir.path, &["diff", "--no-color"]);
    git_output(&dir.path, &["checkout", "--", "circuit.json"]);
    let stray = patch.replace("circuit.json", "fusible.yml");

    let good = json!({"board": "main", "patch": patch, "dry_run": false});
    let bad = json!({"board": "main", "patch": stray, "dry_run": false});
    let replies = session(
        &dir.path,
        &[call(1, "edit_board", bad), call(2, "edit_board", good)],
    );

    assert!(is_error(&replies[0]));
    assert_eq!(structured(&replies[1])["applied"], true);
    assert_eq!(structured(&replies[1])["fuse"], "blown");
    assert_eq!(std::fs::read_to_string(&file).unwrap(), blown_text());
}

#[test]
fn edit_body_needs_a_code_part() {
    let dir = committed_project();

    let replies = session(
        &dir.path,
        &[call(
            1,
            "edit_body",
            json!({"name": "nothing", "content": "x"}),
        )],
    );

    assert!(is_error(&replies[0]));
}
