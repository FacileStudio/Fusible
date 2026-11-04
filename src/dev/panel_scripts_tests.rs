use super::assets::static_response;

fn body_of(path: &str, desktop: bool) -> String {
    let response = static_response(path, None, desktop).unwrap();
    let mut body = String::new();
    std::io::Read::read_to_string(&mut response.into_reader(), &mut body).unwrap();
    body
}

fn content_type(path: &str) -> String {
    let response = static_response(path, None, false).unwrap();
    response
        .headers()
        .iter()
        .find(|h| h.field.equiv("Content-Type"))
        .map(|h| h.value.to_string())
        .unwrap_or_default()
}

#[test]
fn viewer_mounts_the_studio_app_and_links_its_bundle_and_styles() {
    let html = include_str!("../assets/viewer.html");
    assert!(html.contains(r#"<div id="app"></div>"#));
    assert!(html.contains("/studio.bundle.js"));
    assert!(html.contains("/studio.css"));
    assert!(html.contains("fusible-theme"));
}

#[test]
fn desktop_viewer_sets_the_flag_the_bundle_reads() {
    assert!(body_of("/", true).contains("window.FUSIBLE_DESKTOP = true"));
    assert!(!body_of("/", false).contains("FUSIBLE_DESKTOP"));
}

#[test]
fn server_serves_the_favicon_and_the_stylesheet() {
    assert_eq!(content_type("/favicon.svg"), "image/svg+xml");
    assert_eq!(content_type("/favicon.ico"), "image/svg+xml");
    let css = static_response("/studio.css", None, false).unwrap();
    assert_eq!(css.status_code().0, 200);
    assert_eq!(content_type("/studio.css"), "text/css; charset=utf-8");
}

#[test]
fn old_page_scripts_are_gone() {
    for path in ["/project.js", "/git.js", "/bom.js", "/theme.css"] {
        assert!(static_response(path, None, false).is_none(), "{path}");
    }
}
