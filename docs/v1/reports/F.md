# Track F report
Branch: feat/v1-mcp   Last commit: (see git log; this report is the final commit)
## Done
- `fusible mcp [--root] [--print-config bulle|claude|codex]`: stdio JSON-RPC, 13 tools, 2 resource kinds, per-file write safety (1.4) with tests.
- `check --opus [--opus-project]`, `bom --opus`: best-effort Opus tasks, dedup in `.fusible/opus.json`.
- `fusible share nuage --space ... [--preset] [--link] [--expires] [--force]`.
- `docs/agents.md`; fake CLIs in `tests/fixtures/bin/`; tests: `mcp*.rs`, `opus.rs`, `share.rs`.
- Stdout stays protocol-only: on unix the server dup2s fd 1 onto stderr (one small `unsafe` extern call, no new crate), so reused commands that println cannot corrupt the stream.
## Stubbed or partial (wire after D and E merge)
- `explain_rule`: returns `{id, severity}` only. No summary, standard or source_url until `Linter::describe()` (D).
- `check` and `edit_board` diagnostics: `{rule, message, help, severity}` only. No `standard`, `source_url`, `elements`, `fix` (D). The Opus dedup key therefore uses the message, not elements.
- `list_nets`: `declared` is always `{}` (needs D's `nets:` manifest key).
- `body_new`: always `isError` "body_new needs Fusible 1.0 bodies" (E).
- `edit_body`: write safety works, but no mechanical rules run: `fuse` is `null`, `diagnostics` is `[]` (E).
- `project_info`: body `material` and `process` are `null`; `kind` is "code" for a `.ts` source (E).
- `build_body`: `bbox` is `null`; `mass_g` comes from the STL when one exists.
- `check` does not run the manifest `links` and `assembly` checks that `fusible check` runs.
- Opus: description lacks `<standard> <source_url>` (D). Manifest key `opus: {project}` not added; `--opus-project` flag only.
## Gates
- cargo fmt --check: pass
- cargo clippy --all-targets -D warnings: pass
- cargo test --locked: all pass (375 unit, plus mcp 7, mcp_edit 5, mcp_edge 5, mcp_escape 2, opus 5, share 6)
- filet check: clean
- smoke run (initialize + tools/list via `cargo run -q -- mcp --root tests/fixtures/mcp`): returns both replies
- bun builds/tests, no-border grep: not applicable (no UI)
## Not done / blocked, ambiguities to decide
- `nuage sync --json`: the spec says to pass `--json`, but the nuage skill says daemon commands (`sync`) do NOT support `--json`. I followed the spec. If real nuage rejects it, drop the flag in `src/commands/share/nuage.rs`. Same doubt for `nuage share --expires` (skill documents `-e`).
- `opus task add --json` output shape is unverified (I never ran opus). The parser reads `id` or `task.id`.
- `opus task add` parses inline syntax in the title (+project, *label, @user, !p). I strip those leading markers from words. Natural-language dates in a message could still be read as a due date.
- Array outputs (`list_components`, `list_nets`) are wrapped as `{"components":[...]}` / `{"nets":[...]}` because MCP structuredContent must be an object. The spec showed bare arrays.
- Argument errors return `isError` tool results; only an unknown tool or missing name is -32602.
- Write safety step 3 is stricter than the spec: a file the session wrote is allowed only while its bytes still equal what the session wrote.
- Patches name paths relative to the project root; a project that sits in a git subdirectory is untested.
## Notes for the integrator
- Shared-file edits: `src/cli.rs` (Check gets a flattened `OpusArgs`; `Mcp`, `Share` variants), `src/cli/args.rs` (BomArgs gets `opus`), `src/main.rs` (Check arm, Mcp exempt from the cwd manifest load, Init now via `dispatch::init`), `src/dispatch.rs` (`check` takes `OpusArgs`), `src/commands/bom.rs` (`report` is `pub(crate)`, `--opus` hook). D's `--json` on Check will conflict on the same variant.
- `src/commands/mcp/report.rs` is the quiet lint path; when D adds fields to `Diagnostic`, extend `diag_json.rs`.
- `fusible bom` and the `bom` tool create `bom.yml` when missing (existing behavior).
