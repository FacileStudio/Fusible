//! A failed build must not pass off the Circuit JSON that was already on disk.

use std::fs;
use std::os::unix::fs::PermissionsExt;

use super::detect::TempDirGuard;
use crate::tscircuit::{Runner, RunnerError, RunnerKind};

#[test]
fn a_failed_build_rejects_the_circuit_written_just_before_it() {
    let temp = TempDirGuard::new("fusible_stale_circuit");
    let tsci = temp.path.join("fake-tsci");
    fs::write(&tsci, "#!/bin/sh\necho 'compile error' >&2\nexit 1\n").unwrap();
    fs::set_permissions(&tsci, fs::Permissions::from_mode(0o755)).unwrap();
    let ws = temp.path.join("ws");
    fs::create_dir_all(ws.join("dist")).unwrap();
    fs::write(ws.join("package.json"), "{}").unwrap();
    fs::write(ws.join("board.tsx"), "export default 1").unwrap();
    fs::write(ws.join("dist/circuit.json"), "[]").unwrap();
    let runner = Runner::new(RunnerKind::Global(tsci));
    let busy = |e: &RunnerError| format!("{e:?}").contains("ExecutableFileBusy");

    let mut tries = (0..200).map(|_| runner.build(&ws.join("board.tsx"), None));
    let result = tries.find(|r| !r.as_ref().is_err_and(busy)).unwrap();

    assert!(result.is_err(), "{result:?}");
}
