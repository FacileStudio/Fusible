use std::path::{Path, PathBuf};

use super::jobs::{select_jobs, Job};
use crate::config::FusibleConfig;
use crate::project::Project;

#[test]
fn a_body_name_wins_over_a_local_directory_of_the_same_name() {
    assert!(std::path::Path::new("src").is_dir());
    let cfg = FusibleConfig::from_str("bodies: {src: {source: parts/src.step}}\n").unwrap();
    let project = Project::from_parts(Path::new("."), Some((PathBuf::from("fusible.yml"), cfg)));
    let jobs = select_jobs(&project, Some(PathBuf::from("src"))).unwrap();
    assert!(matches!(jobs.as_slice(), [Job::Authored { name, .. }] if name == "src"));
}
