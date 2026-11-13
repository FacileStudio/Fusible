//! `GET /api/bodies/<name>.glb`: the GLB of an authored body or a code part.
//!
//! `GET /api/bodies` lists every body, see [`list`].

pub(super) mod authored;
pub(super) mod authored_cache;
#[cfg(test)]
pub(super) mod authored_cache_tests;
#[cfg(test)]
pub(super) mod authored_support;
#[cfg(test)]
pub(super) mod authored_tests;
pub(super) mod body;
#[cfg(test)]
pub(super) mod body_tests;
pub(super) mod list;

use std::path::Path;

use miette::Diagnostic;
use tiny_http::Request;

use self::authored::{authored_glb, Failure};
use self::body::no_store;
use crate::commands::body::{code_glb, convert_to_glb, generator_dir};
use crate::dev::http::assets::http_response;
use crate::dev::http::server::percent_decode;
use crate::project::Project;

use self::list::bodies_json;
use super::api::{error_body, respond_json};

const PREFIX: &str = "/api/bodies/";

/// Extracts the body name from `/api/bodies/<name>.glb`.
pub(super) fn body_name(path: &str) -> Option<String> {
    percent_decode(path.strip_prefix(PREFIX)?.strip_suffix(".glb")?)
}

fn describe(error: crate::commands::body::BodyError) -> String {
    match error.help() {
        Some(help) => format!("{error}: {help}"),
        None => error.to_string(),
    }
}

/// Hides where files live: the cache directory becomes `<cache>`, the project directory `.`.
pub(super) fn scrub(message: &str, root: &Path, cache: &Path) -> String {
    let root_prefix = format!("{}{}", root.display(), std::path::MAIN_SEPARATOR);
    message
        .replace(&cache.display().to_string(), "<cache>")
        .replace(&root_prefix, "")
        .replace(&root.display().to_string(), ".")
}

fn build(path: &str, project: &Project) -> Result<Vec<u8>, Failure> {
    let bad = |status, message: &str| Failure::new(status, message);
    let name = body_name(path).ok_or_else(|| bad(400, "bad body name"))?;
    let (manifest, cfg) = project
        .found()
        .ok_or_else(|| bad(404, "no fusible.yml found"))?;
    let root = manifest.parent().unwrap_or(Path::new("."));
    let cache = generator_dir();
    authored_glb(
        cfg,
        manifest.parent(),
        &name,
        &cache.join("authored"),
        |input, format, output| match format {
            "ts" => code_glb(project, &name, root, output).map_err(describe),
            _ => convert_to_glb(input, format, output).map_err(describe),
        },
    )
    .map_err(|failure| Failure::new(failure.status, scrub(&failure.message, root, &cache)))
}

/// Answers `request` for the body named in `path`, on the thread the server gave this request.
pub(super) fn serve(request: Request, path: &str, project: &Project) {
    let response = match build(path, project) {
        Ok(glb) => http_response(200, "model/gltf-binary", glb),
        Err(failure) => {
            let body = serde_json::json!({ "error": failure.message }).to_string();
            http_response(failure.status, "application/json", body.into_bytes())
        }
    };
    let _ = request.respond(no_store(response));
}

/// Answers `GET /api/bodies` with the list of bodies of the manifest.
pub(super) fn list(request: Request, project: &Project) {
    match project.found() {
        Some((manifest, cfg)) => {
            let root = manifest.parent().unwrap_or(Path::new("."));
            respond_json(request, 200, bodies_json(cfg, root));
        }
        None => respond_json(request, 404, error_body("no fusible.yml found")),
    }
}
