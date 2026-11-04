# Track I report
Branch: feat/v1-fab   Last commit: see `git log -1` (this report is the last commit)
## Done
- Gerber copper pours: `pcb_copper_pour` (rect, polygon, brep with inner rings) now emits G36 regions on its layer. Clear-polarity cutouts surround pads, plated holes, vias and traces of other nets (0.2 mm), the pour is clipped to the board outline (outside region cut-in plus a 0.4 mm clear stroke along the edge). Net identity comes from `subcircuit_connectivity_map_key` via `pcb_port` -> `source_port` / `source_net` / `source_trace`. Same-net copper connects solidly: no thermal relief in v1. Pastille `B_Cu.gbr`: 55 G36 (was 0).
- Rule `autoroute` (error): one diagnostic per `*_error` element, plus "board has nets but no routed traces". Thermalpad hint added to `help` when the message contains `thermalpad`.
- Rule `qfn_footprint_span` (warning), reads `footprinter_string` (on `cad_component`) or `footprint`. The field exists in real circuit JSON.
- `export -f <runner format>` without `-o` now writes `<out_dir>/<entry stem>.<ext>` (dist/).
- `--preset fab` writes `bom.csv` and `pnp.csv` next to the gerbers.
- `fusible release [TARGET] [-o dist/release] [--allow-warnings]`: check all boards, refuse on warnings unless allowed, generate and verify pours in memory, then write bundle and `RELEASE.txt` (version, git revision, verdict, sha256 via `sha256sum`). Exit 1 blown or warnings, 2 otherwise; nothing is written on failure.
- `doctor` lists every `fusible` on PATH, warns on more than one, adds `path_installs:[{path,version,active}]` to `--json`. Exit code unchanged.
## Gates (paste the last lines of each)
- cargo fmt --check: ok
- cargo clippy: Finished, no warnings (-D warnings)
- cargo test --locked: 459 passed, 0 failed
- filet check: no output (clean)
- bun builds/tests: not applicable
- no-border grep: not applicable (no UI paths)
- Pastille: `fusible export --preset fab -o /tmp/claude-1000/pastille-fab` then `unzip -p ... '*B_Cu*' | rg -c G36` -> 55
## Not done / blocked
- `standard: "tscircuit"` on the `autoroute` diagnostic: `Diagnostic` has no `standard` field until track D lands. Set it after merge.
- Mask openings for pours with `covered_with_solder_mask: false` are not emitted (an opening over the whole pour would expose foreign traces). Pastille pours are covered.
- NPTH (`pcb_hole`) clearance cutouts inside rect/polygon pours are not generated. Brep pours already carry their own inner rings.
- The `autoroute` "skip types another rule handles" const is omitted: no existing rule handles any `*_error` type.
- The `-o` default change in `src/cli/args.rs` was not needed (resolved in `formats.rs`).
## Notes for the integrator (conflicts, contract questions)
- Ambiguity: `pnp.csv` x/y use `pcb_component.center` as the track file says. tscircuit also gives `footprint_origin`, which differs for asymmetric footprints. Fab houses usually expect the footprint origin; decide before ordering assembly.
- Ambiguity: `release` refuses on warnings by default (reading `--allow-warnings` literally). `release` takes an optional `[TARGET]` like `check`/`bom`; the track file lists none.
- Shared-file edits: `src/linter.rs` (two `.with_rule` lines and imports), `src/linter/rules.rs`, `src/cli.rs` (`Release` variant), `src/cli/args.rs` (`ReleaseArgs`), `src/main.rs` (one arm), `src/dispatch.rs` (mod + re-export), `src/commands/mod.rs`.
- `tests/assembly.rs` fixture gained a `pcb_trace` element: its circuit has nets but no traces, which the new `autoroute` rule blocks.
- `doctor` on this machine finds 4 fusible binaries (mise 0.14.3 shadows 0.20.9).
