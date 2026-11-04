//! Client configuration snippets printed by `fusible mcp --print-config`.

use crate::cli::McpClient;

const BULLE: &str = "sources:\n  mcp:\n    fusible:\n      command: fusible\n      args: [mcp]\n";
const CLAUDE: &str = "claude mcp add fusible -- fusible mcp\n";
const CODEX: &str = "[mcp_servers.fusible]\ncommand = \"fusible\"\nargs = [\"mcp\"]\n";

/// Returns the snippet that registers `fusible mcp` in `client`.
pub fn snippet(client: McpClient) -> &'static str {
    match client {
        McpClient::Bulle => BULLE,
        McpClient::Claude => CLAUDE,
        McpClient::Codex => CODEX,
    }
}
