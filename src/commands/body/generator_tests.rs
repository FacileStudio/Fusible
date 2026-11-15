use std::process::Stdio;
use std::time::{Duration, Instant};

use super::generator::{timeout_from, wait_or_kill};

fn spawn(unix: &str, windows: &str) -> std::process::Child {
    crate::test_shell::shell(unix, windows)
        .stdout(Stdio::null())
        .spawn()
        .unwrap()
}

#[test]
fn a_script_that_runs_too_long_is_killed() {
    let mut child = spawn("sleep 30", "ping -n 31 127.0.0.1 >nul");
    let start = Instant::now();
    assert!(wait_or_kill(&mut child, Duration::from_millis(200)).is_none());
    assert!(start.elapsed() < Duration::from_secs(5));
    assert!(child.try_wait().unwrap().is_some(), "the child was reaped");
}

#[test]
fn a_script_that_finishes_in_time_reports_its_status() {
    let mut child = spawn("exit 3", "exit 3");
    let status = wait_or_kill(&mut child, Duration::from_secs(10)).unwrap();
    assert_eq!(status.code(), Some(3));
}

#[test]
fn the_deadline_takes_a_positive_integer_and_ignores_the_rest() {
    assert_eq!(timeout_from(Some("600")), Duration::from_secs(600));
    for bad in [None, Some("0"), Some("-5"), Some("soon")] {
        assert_eq!(timeout_from(bad), Duration::from_secs(120));
    }
}
