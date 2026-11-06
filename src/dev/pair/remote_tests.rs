use std::time::Instant;

use tiny_http::Method;

use super::claim::{claim, start_pairing, ClaimBody};
use super::remote::check;
use super::tests::temp_store;
use super::PairState;

fn paired() -> (PairState, String) {
    let pair = PairState::new(Some(temp_store()));
    let now = Instant::now();
    let (token, _) = start_pairing(&pair, now, 0);
    let body = ClaimBody { token, name: None };
    let reply = claim(&pair, &body, super::claim_tests::PEER, now, 0).unwrap();
    let json: serde_json::Value = serde_json::from_str(&reply).unwrap();
    let secret = json["device_token"].as_str().unwrap().to_string();
    (pair, secret)
}

fn run(pair: &PairState, method: Method, path: &str, bearer: Option<&str>) -> Result<(), u16> {
    check(pair, &method, path, bearer, Instant::now())
}

#[test]
fn no_bearer_and_a_wrong_bearer_give_401() {
    let (pair, _) = paired();
    assert_eq!(
        run(&pair, Method::Get, "/api/mobile/summary", None),
        Err(401)
    );
    assert_eq!(
        run(&pair, Method::Get, "/api/mobile/summary", Some("nope")),
        Err(401)
    );
}

#[test]
fn a_post_with_a_good_bearer_is_405() {
    let (pair, secret) = paired();
    assert_eq!(
        run(&pair, Method::Post, "/api/run", Some(&secret)),
        Err(405)
    );
    assert_eq!(
        run(&pair, Method::Delete, "/api/pair/devices/x", Some(&secret)),
        Err(405)
    );
}

#[test]
fn a_path_off_the_allowlist_is_403() {
    let (pair, secret) = paired();
    for path in [
        "/api/git",
        "/api/pair/devices",
        "/api/runs",
        "/events",
        "/api/bodies/a/b.glb",
    ] {
        assert_eq!(
            run(&pair, Method::Get, path, Some(&secret)),
            Err(403),
            "{path}"
        );
    }
}

#[test]
fn allowed_paths_pass_with_a_good_bearer() {
    let (pair, secret) = paired();
    let paths = [
        "/api/mobile/summary",
        "/api/circuit",
        "/api/bom",
        "/api/bodies",
        "/api/bodies/bracket.glb",
        "/api/rules",
        "/api/project",
    ];
    for path in paths {
        assert_eq!(
            run(&pair, Method::Get, path, Some(&secret)),
            Ok(()),
            "{path}"
        );
    }
}

#[test]
fn claim_needs_no_bearer_but_only_by_post() {
    let (pair, _) = paired();
    assert_eq!(run(&pair, Method::Post, "/api/pair/claim", None), Ok(()));
    assert_eq!(run(&pair, Method::Get, "/api/pair/claim", None), Err(401));
}

#[test]
fn a_revoked_device_is_refused() {
    let (pair, secret) = paired();
    let id = pair.devices.lock().unwrap().authenticate(&secret).unwrap();
    assert!(pair.devices.lock().unwrap().remove(&id));
    assert_eq!(
        run(&pair, Method::Get, "/api/circuit", Some(&secret)),
        Err(401)
    );
    assert!(!pair.devices.lock().unwrap().remove(&id));
}
