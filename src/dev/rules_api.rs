//! `GET /api/rules`: the registered rules, their standards and whether the manifest feeds them.

use tiny_http::Request;

use super::api::respond_json;
use crate::config::DesignRules;
use crate::linter::Linter;

/// The JSON list of every registered rule for the manifest nearest to the working directory.
pub(super) fn rules_json(design: Option<&DesignRules>) -> String {
    let rules = Linter::default().describe(design);
    serde_json::to_string(&rules).unwrap_or_else(|_| "[]".to_string())
}

/// Serves the rule list.
pub(super) fn serve(request: Request) {
    respond_json(request, 200, rules_json(DesignRules::nearest().as_ref()));
}
