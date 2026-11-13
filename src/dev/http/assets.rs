use std::io::Cursor;
#[cfg(not(all(studio_bundle, not(debug_assertions))))]
use std::path::PathBuf;

use tiny_http::{Header, Response};

const VIEWER_HTML: &str = include_str!("../../assets/viewer.html");
const FAVICON_SVG: &str = include_str!("../../../assets/favicon.svg");
const MISSING_BUNDLE: &str = "// studio.bundle.js not found — run `bun run build` in studio/";

/// A buffered HTTP response as served by the dev server.
pub(in crate::dev) type HttpResponse = Response<Cursor<Vec<u8>>>;

/// Returns the studio bundle embedded at compile time in release builds.
#[cfg(all(studio_bundle, not(debug_assertions)))]
pub(in crate::dev) fn load_studio_bundle() -> Option<String> {
    Some(
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/assets/studio.bundle.js"
        ))
        .to_string(),
    )
}

/// Reads the studio bundle from the source tree so a `bun run build` is picked up without recompiling.
#[cfg(not(all(studio_bundle, not(debug_assertions))))]
pub(in crate::dev) fn load_studio_bundle() -> Option<String> {
    let bundle_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/assets/studio.bundle.js");
    std::fs::read_to_string(&bundle_path).ok()
}

/// Returns the studio stylesheet embedded at compile time in release builds.
#[cfg(all(studio_bundle, not(debug_assertions)))]
pub(in crate::dev) fn load_studio_css() -> Option<String> {
    Some(
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/assets/studio.css"
        ))
        .to_string(),
    )
}

/// Reads the studio stylesheet from the source tree so a `bun run build` is picked up without recompiling.
#[cfg(not(all(studio_bundle, not(debug_assertions))))]
pub(in crate::dev) fn load_studio_css() -> Option<String> {
    let css_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/assets/studio.css");
    std::fs::read_to_string(&css_path).ok()
}

/// Builds a response with `status`, a `Content-Type` header and `body`.
pub(in crate::dev) fn http_response(
    status: u16,
    content_type: &str,
    body: Vec<u8>,
) -> HttpResponse {
    let header = Header::from_bytes(&b"Content-Type"[..], content_type.as_bytes())
        .expect("content type is a valid header value");
    Response::from_data(body)
        .with_header(header)
        .with_status_code(status)
}

/// Returns the viewer page; under the desktop shell it carries the flag the bundle reads.
fn viewer_html(desktop: bool) -> String {
    if !desktop {
        return VIEWER_HTML.into();
    }
    VIEWER_HTML.replacen(
        "<head>",
        "<head>\n  <script>window.FUSIBLE_DESKTOP = true;</script>",
        1,
    )
}

/// Serves the embedded viewer, favicon, studio stylesheet and studio bundle; `None` for other paths.
pub(in crate::dev) fn static_response(
    path: &str,
    bundle: Option<String>,
    desktop: bool,
) -> Option<HttpResponse> {
    let response = match path {
        "/" | "/index.html" => http_response(
            200,
            "text/html; charset=utf-8",
            viewer_html(desktop).into_bytes(),
        ),
        "/studio.bundle.js" => bundle_response(bundle),
        "/studio.css" => http_response(
            200,
            "text/css; charset=utf-8",
            load_studio_css().unwrap_or_default().into_bytes(),
        ),
        "/favicon.ico" | "/favicon.svg" => http_response(200, "image/svg+xml", FAVICON_SVG.into()),
        _ => return None,
    };
    Some(response)
}

fn bundle_response(bundle: Option<String>) -> HttpResponse {
    let content_type = "application/javascript; charset=utf-8";
    match bundle {
        Some(content) => http_response(200, content_type, content.into_bytes()),
        None => http_response(404, content_type, MISSING_BUNDLE.into()),
    }
}
