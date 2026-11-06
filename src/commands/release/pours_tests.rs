use std::collections::BTreeMap;

use super::{verify, ReleaseError};

#[test]
fn unparseable_circuit_json_is_an_error_not_a_pass() {
    let files = BTreeMap::new();
    let result = verify("b", "not json", &files);
    assert!(matches!(result, Err(ReleaseError::Unreadable(_))));
}
