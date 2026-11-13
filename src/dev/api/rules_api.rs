//! `GET /api/rules`: the registered rules, their standards and whether the manifest feeds them.

use tiny_http::Request;

use crate::config::DesignRules;
use crate::dev::api::respond_json;
use crate::linter::catalog;
use crate::project::Project;

/// The JSON list of every registered rule for the manifest nearest to the working directory.
pub(in crate::dev) fn rules_json(design: Option<&DesignRules>) -> String {
    let rules = catalog(design);
    serde_json::to_string(&rules).unwrap_or_else(|_| "[]".to_string())
}

/// Serves the rule list.
pub(in crate::dev) fn serve(request: Request, project: &Project) {
    respond_json(request, 200, rules_json(project.design_rules()));
}
