//! Integration tests for `doctor --json`.

mod common;

use common::fusible;
#[cfg(unix)]
use common::TestDir;

#[test]
fn doctor_json_lists_bun_node_tsci_and_exits_zero_with_nothing_installed() {
    let out = fusible()
        .args(["doctor", "--json"])
        .env("PATH", "")
        .env_remove("FUSIBLE_BUN")
        .output()
        .expect("binary should run");
    assert_eq!(out.status.code(), Some(0));
    let json: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    for tool in ["bun", "node", "tsci"] {
        assert!(json[tool].is_null(), "{tool}: {json}");
    }
}

#[cfg(unix)]
#[test]
fn doctor_json_reports_the_bun_named_by_fusible_bun() {
    use std::os::unix::fs::PermissionsExt;
    let fixture = TestDir::new("doctor_json_bun");
    let bun = fixture.path.join("bun");
    std::fs::write(&bun, "#!/bin/sh\necho 1.2.3\n").unwrap();
    std::fs::set_permissions(&bun, std::fs::Permissions::from_mode(0o755)).unwrap();
    let out = fusible()
        .args(["doctor", "--json"])
        .env("PATH", "")
        .env("FUSIBLE_BUN", &bun)
        .output()
        .expect("binary should run");
    assert_eq!(out.status.code(), Some(0));
    let json: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(json["bun"]["path"], bun.to_str().unwrap());
    assert_eq!(json["bun"]["version"], "1.2.3");
}

#[cfg(unix)]
fn stub_fusible(dir: &std::path::Path, version: &str) {
    use std::os::unix::fs::PermissionsExt;
    std::fs::create_dir_all(dir).unwrap();
    let file = dir.join("fusible");
    std::fs::write(&file, format!("#!/bin/sh\necho fusible {version}\n")).unwrap();
    std::fs::set_permissions(&file, std::fs::Permissions::from_mode(0o755)).unwrap();
}

#[cfg(unix)]
#[test]
fn doctor_reports_a_stale_fusible_shadowing_the_current_one() {
    let fixture = TestDir::new("doctor_installs");
    let (old, new) = (fixture.path.join("old"), fixture.path.join("new"));
    stub_fusible(&old, "0.14.3");
    stub_fusible(&new, "0.20.9");
    let path = std::env::join_paths([&old, &new]).unwrap();
    let json_out = fusible()
        .args(["doctor", "--json"])
        .env("PATH", &path)
        .output()
        .unwrap();
    assert_eq!(json_out.status.code(), Some(0));
    let json: serde_json::Value = serde_json::from_slice(&json_out.stdout).unwrap();
    let installs = json["path_installs"].as_array().unwrap();
    assert_eq!(installs.len(), 2);
    assert_eq!(installs[0]["version"], "0.14.3");
    assert_eq!(installs[0]["active"], true);
    assert_eq!(installs[1]["active"], false);

    let text = fusible().arg("doctor").env("PATH", &path).output().unwrap();
    assert_eq!(text.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&text.stdout);
    assert!(stdout.contains("2 fusible binaries on PATH"), "{stdout}");
    assert!(stdout.contains("mise uninstall cargo:fusible"));
}

#[test]
fn doctor_json_has_no_path_installs_when_none_exist() {
    let out = fusible()
        .args(["doctor", "--json"])
        .env("PATH", "")
        .output()
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(json["path_installs"].as_array().unwrap().len(), 0);
}
