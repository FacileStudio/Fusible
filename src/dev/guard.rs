//! Who may talk to the dev server: loopback hosts, and pages served by this server only.

use tiny_http::Request;

use super::pair::{authorize_remote, is_lan_host, is_remote};
use super::server::route_path;
use super::state::State;

const LOOPBACK: [&str; 3] = ["localhost", "127.0.0.1", "[::1]"];

/// Accepts only loopback Host headers bound to the dev server port, blocking DNS rebinding.
pub(super) fn is_allowed_host(host: Option<&str>, port: u16) -> bool {
    host.is_some_and(|h| {
        LOOPBACK
            .iter()
            .any(|name| h.eq_ignore_ascii_case(&format!("{name}:{port}")))
    })
}

/// Accepts a missing Origin, or one that is a loopback page served by this dev server.
pub(super) fn is_allowed_origin(origin: Option<&str>, port: u16) -> bool {
    origin.is_none_or(|o| {
        LOOPBACK
            .iter()
            .any(|name| o.eq_ignore_ascii_case(&format!("http://{name}:{port}")))
    })
}

/// Rejects requests the browser marks as coming from another site, including another local port.
///
/// A missing header (curl, older browsers) and `none` (the user typed the address) pass.
pub(super) fn is_allowed_fetch_site(site: Option<&str>) -> bool {
    site.is_none_or(|s| s.eq_ignore_ascii_case("same-origin") || s.eq_ignore_ascii_case("none"))
}

/// Whether `request` comes from this dev server's own pages or from a non-browser client.
pub(super) fn is_same_origin(request: &Request, port: u16) -> bool {
    let header = |name: &'static str| {
        let found = request.headers().iter().find(|h| h.field.equiv(name));
        found.map(|h| h.value.as_str().to_string())
    };
    is_allowed_origin(header("Origin").as_deref(), port)
        && is_allowed_fetch_site(header("Sec-Fetch-Site").as_deref())
}

/// Decides whether `request` may be handled at all; the error is the status to answer with.
///
/// A remote phone goes through the pairing gate. Everything else keeps the loopback rules.
pub(super) fn admit(request: &Request, state: &State, port: u16) -> Result<(), u16> {
    let found = request.headers().iter().find(|h| h.field.equiv("Host"));
    let host = found.map(|h| h.value.as_str());
    let lan = is_lan_host(host, state.lan_host.as_deref());
    if !lan && !is_allowed_host(host, port) {
        return Err(403);
    }
    if is_remote(request, host, state) {
        return authorize_remote(request, state);
    }
    let path = route_path(request.url());
    let guarded = path.starts_with("/api/") || path == "/events";
    if guarded && !is_same_origin(request, port) {
        return Err(403);
    }
    Ok(())
}
