//! Every tscircuit subprocess shares one deadline and one timeout message.

use std::time::{Duration, Instant};

use crate::test_shell::sleeping;
use crate::tscircuit::deadline::output_within;
use crate::tscircuit::RunnerError;

#[test]
fn a_command_past_the_deadline_is_killed_and_names_the_variable() {
    let started = Instant::now();
    let err = output_within(&mut sleeping(30), Duration::from_millis(200)).unwrap_err();

    assert!(started.elapsed() < Duration::from_secs(10));
    let RunnerError::CommandFailed { code, help, .. } = err else {
        panic!("{err:?}");
    };
    assert_eq!(code, None);
    assert!(help.contains("FUSIBLE_BUILD_TIMEOUT_SECS"), "{help}");
}
