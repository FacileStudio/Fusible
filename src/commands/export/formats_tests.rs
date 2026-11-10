//! Tests for direct format exports.

use super::formats::runner_output;
use super::ExportJob;
use std::path::PathBuf;

fn job(output: Option<&str>) -> ExportJob {
    ExportJob {
        runner: None,
        entry: PathBuf::from("hardware/index.tsx"),
        board_title: "t".into(),
        out_dir: PathBuf::from("dist"),
        output: output.map(PathBuf::from),
        source: None,
        manifest: None,
    }
}

#[test]
fn format_without_output_lands_in_dist() {
    assert_eq!(
        runner_output(&job(None), "glb"),
        PathBuf::from("dist/index.glb")
    );
}

#[test]
fn explicit_output_wins() {
    assert_eq!(
        runner_output(&job(Some("x/y.glb")), "glb"),
        PathBuf::from("x/y.glb")
    );
}
