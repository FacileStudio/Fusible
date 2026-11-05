# Track D report
Branch: feat/v1-rules   Last commit: see `git log -1` (this report is the last commit)

## Done
- `src/parser/geometry.rs`: reads segments, vias, holes and the board from `raw_elements`. Unreadable elements go to `unreadable`; each physics rule reports them as an error.
- Manifest keys `nets`, `fab`, `clearance_class`, `copper`, validated (B3/A5/A6/A7 rejected with the spec message, unknown named fab, non-positive numbers). `DesignRules` built from the config. `fusible init` template has a commented example.
- `LintContext::with_design`, wired into `lint_board` (check, export, build, assembly, body) and the studio pipeline.
- `Diagnostic` and `DevDiagnostic` carry `standard`, `source_url`, `elements`, `fix`; `DevDiagnostic` also carries `severity`. miette `url()` shows `source_url`.
- Rules: `trace_current` (IPC-2152), `voltage_clearance` (IPC-2221B, table in `data/ipc2221b.yml`), `fab_profile` (`data/fab/jlcpcb-2l.yml`, `jlcpcb-4l.yml`), `decoupling_capacitor` 3 mm placement warning. `trace_clearance` reads `fab.min_space` when a fab is set.
- `fusible check --json` and `GET /api/rules`. `Rule` gained `summary`, `standard`, `source_url`, `enabled`; `Linter::describe(design)`.
- Examples: `examples/circuit-trace-current.json`, `circuit-voltage-clearance.json`, `circuit-fab-profile.json`; manifest fixture `tests/fixtures/rules/fusible.yml`; integration tests `tests/physics_rules.rs`.

## Gates (paste the last lines of each)
- cargo fmt --check: pass
- cargo clippy: `-D warnings` clean
- cargo test --locked: 473 passed, 0 failed
- filet check: exit 0
- bun builds/tests: not applicable (no studio changes)
- no-border grep: not applicable (no UI paths)
- `cargo run -- check examples/circuit.json --json`: `{"boards":[{"diagnostics":[],"name":"circuit"}],"fuse":"intact"}`, exit 0

## Not done / blocked
- None blocked. Deviations and ambiguities are listed below.

## Notes for the integrator (conflicts, contract questions)
- Rule count is 12, not 13. The spec says "9 existing plus 4 new", but the bypass distance extends `decoupling_capacitor` (as step 5.4 says: do not duplicate) instead of adding a rule.
- `Diagnostic.standard` and `source_url` are `Option<String>`, not `Option<&'static str>`: `fab_profile` takes both from a profile loaded at runtime. `Rule::standard()` and `source_url()` stay `&'static str`.
- `Rule::summary` has a default that looks the text up in `src/linter/summaries.rs`, so no existing rule file changed (filet's 8-functions-per-file limit).
- `Linter::describe` takes the design: `describe(design: Option<&DesignRules>)`, needed for `enabled`.
- `Linter::run_with_context` kept a rule's severity only for errors: a diagnostic that is already a warning stays a warning, even from an error rule. Needed for the `trace_current` "no routed traces" warning and the bypass distance warning. Warning-only rules behave as before.
- Track F/C conflict: `Commands::Check` is now `Check(CheckArgs)` (new `CheckArgs` in `src/cli/args.rs`) because the extra field pushed `run_command` in `src/main.rs` over filet's 30 lines. Add `--opus` to `CheckArgs`. `dispatch::check` gained a `json: bool` parameter.
- `fab_profile` standard in `/api/rules` is the generic "fab capability profile" with no `source_url`; real diagnostics carry the profile name and URL.
- Ambiguities, decided as follows (change if wrong):
  - Via-to-via is measured pad edge to pad edge (stricter), for all via pairs.
  - Board size accepts either orientation.
  - Pill holes count their smaller dimension as the drill.
  - `trace_current` evaluates every segment and reports the hottest. The message says "narrowest" only when it is also the narrowest, else "hottest".
  - A trace's nets come from `connected_source_net_ids`, then from the nets of its `connected_source_port_ids`. Segments with no resolvable net are skipped by the spacing rules.
  - A declared net absent from the circuit is skipped silently (a project-wide `nets:` applies to every board); only a net that exists with no routed traces warns.
  - `voltage_clearance` skips pairs with equal voltage and only uses pads from `pcb_smtpad`/`source_pad`; plated holes are not copper in this check.
  - Fab pad-to-pad spacing is left to `trace_clearance` (which uses `min_space`); `fab_profile` reports pairs that involve a trace.
- `check --json` reports board diagnostics only. The manifest `links` and `assembly` checks and `--pins-rs` are not part of it (`--json` conflicts with `--pins-rs`). A bare file's board name is the file stem.
- `src/dev/firmware.rs` is unchanged: it runs `PinMuxRule` only, so a design context would be dead plumbing. `check_board`'s pin-binding context is also unchanged for the same reason.
- Nothing in `studio/` reads the new `DevDiagnostic` fields yet; they are optional and `severity` is always present.
