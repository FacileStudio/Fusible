#[cfg(unix)]
use super::copy_tree;
use super::{unique_dir, valid_name};

#[test]
fn escaping_names_are_refused() {
    for bad in [
        "",
        "../../x",
        "/home/u/.ssh",
        "a/b",
        "..",
        ".hidden",
        "a\0b",
        "a\\b",
    ] {
        assert!(valid_name(bad).is_err(), "{bad:?}");
    }
    assert!(valid_name("mcp-fixture").is_ok());
}

#[test]
fn unique_dirs_differ_and_are_private() {
    let (a, b) = (unique_dir().unwrap(), unique_dir().unwrap());
    assert_ne!(a, b);
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = std::fs::metadata(&a).unwrap().permissions().mode();
        assert_eq!(mode & 0o777, 0o700);
    }
}

#[cfg(unix)]
#[test]
fn symlinks_are_not_copied() {
    let (from, to) = (unique_dir().unwrap(), unique_dir().unwrap());
    std::os::unix::fs::symlink("/etc/passwd", from.join("link")).unwrap();
    assert!(copy_tree(&from, &to.join("out")).is_err());
}
