//! Routing of the studio's `/api/` requests to their handlers.

use tiny_http::Request;

use super::serve::serve_static;
use crate::dev::api;
use crate::dev::api::bom;
use crate::dev::api::firmware;
use crate::dev::api::fix_api;
use crate::dev::api::rules_api;
use crate::dev::assembly;
use crate::dev::bodies;
use crate::dev::git;
use crate::dev::live::state::State;
use crate::dev::pair;
use crate::dev::run;
use crate::project::Project;

/// Routes one API request with the project loaded for it, since the manifest can change on disk.
pub(in crate::dev) fn api_route(
    request: Request,
    state: &State,
    port: u16,
    path: &str,
    project: &Project,
) {
    match path {
        "/api/mechanical" => api::body_preview(request, state, project, false),
        "/api/mechanical.glb" => api::body_preview(request, state, project, true),
        "/api/circuit" => api::circuit(request, state),
        "/api/project" => api::project(request, state, project),
        "/api/bom" => bom::serve(request, state, port, project),
        "/api/runs" => api::runs(request, state),
        "/api/rules" => rules_api::serve(request, project),
        p if pair::owns(p) => pair::serve(request, state, p, project),
        p if assembly::owns(p) => assembly::serve(request, p, project),
        "/api/bodies" => bodies::list(request, project),
        p if p.starts_with("/api/bodies/") => bodies::serve(request, p, project),
        "/api/kicad" => api::kicad(request, state),
        "/api/firmware" => firmware::serve(request, state, project),
        "/api/run" => run::api::start_run(request, state, port, project),
        "/api/fix" => fix_api::serve(request, state, port),
        p if git::api::owns(p) => git::api::serve(request, state, port, project),
        _ => serve_static(request, state, path),
    }
}
