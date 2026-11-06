use super::{apply_patch, patch_targets};

fn temp(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("fusible-git-{name}-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn git_reports_the_real_target_of_a_double_prefixed_patch() {
    let dir = temp("targets");
    let patch = "diff --git a/a/x b/a/x\nnew file mode 100644\n";
    assert_eq!(patch_targets(&dir, patch).unwrap(), ["a/x"]);
}

#[test]
fn a_patch_for_another_path_is_refused_and_writes_nothing() {
    let dir = temp("refuse");
    let patch = "diff --git a/a/x b/a/x\nnew file mode 100644\n";
    assert!(apply_patch(&dir, patch, "x").is_err());
    assert!(!dir.join("a/x").exists());
}

#[test]
fn a_patch_for_the_allowed_path_applies_from_stdin() {
    let dir = temp("apply");
    std::fs::write(dir.join("x"), "one\n").unwrap();
    let patch = "diff --git a/x b/x\n--- a/x\n+++ b/x\n@@ -1 +1 @@\n-one\n+two\n";
    apply_patch(&dir, patch, "x").unwrap();
    assert_eq!(std::fs::read_to_string(dir.join("x")).unwrap(), "two\n");
}
