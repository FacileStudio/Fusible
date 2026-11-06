use super::{check_patch_scope, Restore};

#[test]
fn restore_puts_the_original_back_when_the_file_is_untouched() {
    let dir = std::env::temp_dir().join(format!("fusible-restore-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("a.txt");
    std::fs::write(&file, "mine").unwrap();
    let mut guard = Restore::new(file.clone(), b"orig".to_vec(), true);
    guard.record();
    drop(guard);
    assert_eq!(std::fs::read(&file).unwrap(), b"orig");
}

#[test]
fn restore_keeps_a_concurrent_user_edit() {
    let dir = std::env::temp_dir().join(format!("fusible-restore-user-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("a.txt");
    std::fs::write(&file, "mine").unwrap();
    let mut guard = Restore::new(file.clone(), b"orig".to_vec(), true);
    guard.record();
    std::fs::write(&file, "user edit").unwrap();
    drop(guard);
    assert_eq!(std::fs::read(&file).unwrap(), b"user edit");
}

#[test]
fn repeated_prefixes_are_not_stripped() {
    let patch = "diff --git a/a/x b/a/x\nnew file mode 100644\n";
    assert!(check_patch_scope(patch, "x").is_err());
    let ok = "diff --git a/x b/x\n--- a/x\n+++ b/x\n";
    assert!(check_patch_scope(ok, "x").is_ok());
}
