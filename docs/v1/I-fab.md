# Track I: fab correctness, release gate, install path

Read `docs/v1/README.md` first. Source of these items: the Pastille session, `~/.mycelium/memory/tools/fusible.md` (entries dated 2026-11-04).

Worktree: `/home/yann/Code/Kono/Fusible-v1-fab`, branch `feat/v1-fab`.

## Goal

Fix the bugs that let Pastille ship unrouted boards and gerbers with missing copper, and add a release command that refuses to export from a blown board.

## Paths you own

- `src/gerber/` (all), `src/commands/export/`, `src/commands/export.rs`, `src/commands/doctor.rs`, `src/commands/doctor/`
- `src/commands/release.rs` (new), `src/cli.rs`/`src/cli/args.rs` (`Release` variant and the `-o` default only)
- `src/linter/rules/autoroute.rs` (new) plus its one-line registration in `src/linter.rs` `Linter::default()` and its tests. Track D also edits `src/linter.rs`: add only your `.with_rule(...)` line.
- `tests/`, `examples/` (add fixtures only)

Don't touch `studio/`, `desktop/`, `mobile/`, `site/`, `src/commands/body`, `src/commands/assembly`.

## Steps

### 1. Gerber copper pours (top priority)
- `fusible export --preset fab` writes `B_Cu.gbr` with no `G36` region for a `<copperpour>` in 0.14.3 and 0.20.9. The KiCad export carries the `(zone`. Read `src/gerber/layers/` and `src/gerber/writer.rs` (`G36/G37` region emitter exists). Find where `pcb_copper_pour` is skipped.
- Circuit JSON shapes to handle: `{"type":"pcb_copper_pour","shape":"rect"|"polygon"|"brep","layer":"bottom","source_net_id":..., "covered_with_solder_mask":bool}`; rect has `center`, `width`, `height`; polygon has `points`; brep has `brep_shape.outer_ring.vertices` and `inner_rings`. Check real data first: `rg -o '"type":"pcb_copper_pour"[^}]*' <a Pastille circuit.json>` (`/home/yann/Code/Facile/Pastille/hardware/dist/index/circuit.json`).
- Emit a filled region with clear polarity cutouts (`%LPC*%` / `%LPD*%`) around pads and traces of other nets, using the board's clearance (default 0.2 mm, or the `fab` profile `min_space` when track D is merged; use 0.2 now). Holes of the same net get thermal-free solid connection (no thermal relief in v1; document it).
- Clip the pour to the board outline.
- Tests in `src/gerber/tests/`: a rect pour produces `G36*`/`G37*` on the right layer; a pad of another net inside the pour produces a clear region; a pour of the same net as a pad does not.
- Verify on Pastille: export, then `unzip -p <zip> *B_Cu.gbr | rg -c G36`. The count must be greater than 0.

### 2. Fuse blows on silent routing failure (new rule `autoroute`)
- Pastille v0.2.2 and v0.3.0 shipped unrouted boards because the fuse stayed intact while tscircuit skipped autorouting.
- New rule `autoroute` (error): scan `circuit.raw_elements` for any element whose `type` ends in `_error` (for example `pcb_autorouting_error`, `pcb_placement_error`, `source_trace_not_connected_error`, `pcb_pad_pad_clearance_error`). One diagnostic per element, with its `message` field. Also: if the circuit has `pcb_*` elements, at least one `source_net` or `source_trace`, and zero `pcb_trace`, emit `"board has nets but no routed traces"`.
- Existing rules may already report some `*_error` types (`rg -n "_error" src/linter`). Don't duplicate: skip types another rule handles, listed in a const.
- `standard: "tscircuit"`, `help`: `"fix the placement or routing error; fab files from an unrouted board are unusable"`.
- Tests: element `{"type":"pcb_placement_error","message":"x"}` blows; a clean circuit passes; nets with zero traces blows; every existing example that is meant to pass still passes (`cargo test`).

### 3. tscircuit gotcha lints (warnings)
- `qfn_footprint_span`: a footprint string matching `qfn\d+_w([\d.]+)_h([\d.]+)_p([\d.]+)` with no `_pw`/`_pl` and w or h less than 3.5 → warning `"qfn w/h is the copper span, not the body; corner pads may short"`. Read footprint strings from `raw_elements` (`pcb_component` / `source_component` `footprint` field if present; check the real field with `rg -o '"footprint":"[^"]*"' <circuit.json> | sort -u`). If the field is absent from circuit JSON, skip this lint and say so in your report.
- `thermalpad_unconnected`: covered by item 2 (`source_trace_not_connected_error`). Add the hint `"a thermalpad only connects when the chip has a pinLabel named thermalpad"` to that diagnostic's `help` when the message contains `thermalpad`.

### 4. `fusible export` default output
- `-f glb` (and any format) with no `-o` must write into `dist/` (project `dist/`), not next to the entry file. Find the output resolution in `src/commands/export/formats.rs`. Add a test.

### 5. `--preset fab` must emit BOM and pick-and-place
- Help text says "gerbers + bom + pnp" but only gerbers come out. Write `bom.csv` (reuse `src/bom`, CSV format that `fusible bom` already supports) and `pnp.csv` (designator, value, package, x, y, rotation, side from `pcb_component` data: `center`, `rotation`, `layer`). Put both next to the gerbers in `dist/fab/`.

### 6. `fusible release`
- New command: `fusible release [--out dist/release] [--allow-warnings]`.
- Steps: `check` every board (blown → exit 1, copy nothing); then require each board to have routed traces (rule `autoroute`); export preset `fab`; verify that every copper layer file that has a `pcb_copper_pour` in the circuit contains `G36` (else exit 2 with the layer name); write a `RELEASE.txt` with fusible version, board names, fuse verdict per board, git revision, and the list of files with SHA-256 (write a tiny SHA-256 using the `sha2` crate only if track C added it; otherwise `sha256sum` via `std::process::Command`, skipping on failure).
- It never writes fab files when any check fails.
- Tests in `tests/release.rs`: blown example → exit 1 and no output dir; intact example → exit 0 and `RELEASE.txt` exists.

### 7. One install path (`fusible doctor`)
- Pastille hit a PATH trap: mise's `cargo-fusible` 0.14.3 shadowed `~/.local/bin/fusible` 0.20.9.
- Extend `doctor`: find every `fusible` on `PATH` (`std::env::split_paths`, check each dir for the file), run `--version` on each, and warn when more than one exists or versions differ: `"2 fusible binaries on PATH: <path> (0.14.3) shadows <path> (0.20.9)"`, with the fix `"remove the older one (mise uninstall cargo:fusible) or reorder PATH"`. Add the result to `doctor --json` as `path_installs:[{path,version,active}]`. Keep exit behaviour: warnings don't change the exit code.
- Do not add a self-updater (suite standard).

## Gates
```sh
cargo fmt --all -- --check && cargo clippy --all-targets -- -D warnings && cargo test --locked
filet check
cd /home/yann/Code/Facile/Pastille && /home/yann/Code/Kono/Fusible-v1-fab/target/debug/fusible export --preset fab -o /tmp/claude-1000/pastille-fab 2>&1 | tail -5; unzip -p /tmp/claude-1000/pastille-fab/*.zip '*B_Cu*' 2>/dev/null | rg -c 'G36'
```
Run the Pastille check read-only: pass `-o` to a temp dir and never modify Pastille's files.

## Exit criteria
- B_Cu contains a pour region on the Pastille board; unrouted boards blow the fuse; `release` refuses blown boards; `export` defaults to `dist/`; fab preset has BOM and PnP; `doctor` reports duplicate installs.
- Report written to `docs/v1/reports/I.md`.
