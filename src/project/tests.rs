use super::Project;

fn temp_dir(tag: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("fusible-project-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("sub")).unwrap();
    dir
}

#[test]
fn loads_the_nearest_manifest_and_its_root() {
    let dir = temp_dir("found");
    std::fs::write(dir.join("fusible.yml"), "name: demo\nboards:\n  main: {}\n").unwrap();
    let project = Project::load(&dir.join("sub")).unwrap();
    let (manifest, config) = project.found().unwrap();
    assert_eq!(manifest.file_name().unwrap(), "fusible.yml");
    assert_eq!(config.name.as_deref(), Some("demo"));
    assert_eq!(project.root, std::path::absolute(&dir).unwrap());
    assert!(project.design_rules().is_some());
    std::fs::write(dir.join("fusible.yml"), "name: renamed\n").unwrap();
    assert_eq!(project.reload().config.name.as_deref(), Some("renamed"));
    std::fs::write(dir.join("fusible.yml"), "boards: [oops\n").unwrap();
    assert!(Project::load(&dir).is_err());
    assert!(project.reload().found().is_none());
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn a_missing_manifest_gives_an_empty_project() {
    let dir = temp_dir("none");
    let project = Project::load(&dir).unwrap();
    assert!(project.found().is_none());
    assert!(project.manifest_dir().is_none());
    assert!(project.design_rules().is_none());
    assert_eq!(project.root, dir);
    let _ = std::fs::remove_dir_all(dir);
}
