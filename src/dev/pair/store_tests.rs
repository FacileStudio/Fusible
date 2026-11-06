use std::os::unix::fs::PermissionsExt;
use std::time::Instant;

use super::{Device, Devices};
use crate::dev::pair::tests::temp_store;

#[test]
fn devices_json_is_written_with_mode_0600() {
    let path = temp_store();
    let mut devices = Devices::load(Some(path.clone()));
    let device = Device {
        device_id: "id".into(),
        name: "phone".into(),
        token_sha256: "h".into(),
        paired_at: "t".into(),
        last_seen: "t".into(),
    };
    devices.add(device, Instant::now()).unwrap();
    let mode = std::fs::metadata(&path).unwrap().permissions().mode();
    assert_eq!(mode & 0o777, 0o600);
}
