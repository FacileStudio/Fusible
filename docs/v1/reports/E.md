# Track E report
Branch: feat/v1-mech   Last commit: see `git log -1` (this report is the last commit)

## Done
- Config: `BodyConfig.material` and `process`; validation against the material library (unknown material, unknown process, process not supported by the material). `BodyConfig::is_code()` for `.ts` sources. `new` is a reserved body name.
- Materials: `src/bom/materials.yml` (embedded), `src/bom/materials.rs`, `stl_mass(path, material, process)` (fill 0.4 for fdm, 1.0 otherwise; `stl_grams` kept as the PLA/FDM wrapper). The BOM print lines carry material, process, grams and an estimated price (`~`) from `cost_per_kg`; the table shows "3D print: bracket (al6061 cnc, 17.1 g)".
- Code parts: `Job::Code`, `src/commands/body/part.rs`, runner `src/assets/part.ts` plus `part_board.ts`, `part_cad.ts`, `part_cut.ts`, `part_hw.ts`, `part_ops.ts` (split to fit filet), types in `src/assets/part.d.ts`. Outputs STEP, STL, GLB and BREP. STEP header date is fixed to 2000-01-01, and a test asserts byte-identical STEP over two builds. Features are cached in `<out_dir>/<name>.features.json`.
- `fusible body new <name> [--board] [--material petg] [--process fdm]`: writes `mech/<name>.ts`, `mech/fusible.d.ts`, and a `bodies.<name>` entry (text edit using `yaml_lines.rs`; new `new_manifest.rs`). Exit 2 when the name exists in boards, firmware or bodies.
- Enclosure features: `enclosure.ts` prints the same `{"features":[...]}` line (wall, clearance, boss outer/pilot, vent w); outputs unchanged. Cached on `fusible body`.
- Mechanical rules (`mech_rules.rs`, `mech_rules_parts.rs`, `mech_rules_data.rs`): `mech.wall_thickness`, `mech.boss`, `mech.port_clearance`, `mech.vent_width`, `mech.standoff_height`, `mech.component_clearance`, `mech.draft`. Run by `fusible check` (`body::verify_mech`, called from `check_manifest_assembly`). Warning for code parts never built (and for a source newer than its cache).
- `GET /api/bodies` (`src/dev/bodies_list.rs`, one arm in `server.rs`); `/api/bodies/<name>.glb` builds code parts on demand through the existing cache.
- Assembly: a code part is built to STEP in `parts_dir` and placed in the board frame (no offset).
- `examples/mech/` (own `circuit.json`, `case.ts`, `bracket.ts`, `bad.ts`), `tests/mech.rs`, `tests/mech_new.rs`, unit tests per rule.
- Addendum 1: kit openings only for connectors (`kit_parts.rs`). Addendum 2: `VERIFIED_PARTS` in `kit_levels.rs`. Addendum 3: `assembly.hole_min_diameter` (default 1.0), exempt holes under a footprint (`Role::HoleNear.exempt`). Addendum 4: `src/assets/slim.ts` (+ `glb.ts`, `slim_pack.ts`), used by `convert.ts`, `assembly.ts` and `part.ts`; a bun-gated test asserts the primitive count drops (to 1).

### occt-wasm 5.7.1 functions behind each `Cad` method
| Cad method | occt-wasm calls |
|---|---|
| `box` | `makeBox` |
| `cylinder` | `makeCylinder` |
| `extrude` | `makeLineEdge`, `makeWire`, `makeFace`, `extrude`, `translate` |
| `union` | `fuseAll` |
| `cut` | `cutAll` |
| `intersect` | `common` |
| `move` | `translate` |
| `rotate` | `rotate` (axis through the origin) |
| `fillet` | `fillet` on `getSubShapes(shape, "edge")`: rounds every edge |
| `shell` | `makeLineEdge`, `makeWire`, `offsetWire2D`, `makeFace`, `extrude`, `translate`, `cut` |
| `boss`, `standoff` | `makeCylinder`, `translate`, `cut` |
| `ventSlots`, `cutout` | `makeBoxFromCorners`, `fuseAll` |
| `draft` | none: records the feature only |
Runner: `exportStep`, `exportStl`, `toBREP`, `createXCAFDocument` / `addShape` / `exportGLTF`, `getBoundingBox`, `getVolume`, `common` (clearance probe).
Omitted: nothing. `draft` (face draft) and `draftPrism` exist in occt-wasm but `cad.draft(deg)` is declared as a recorded feature only, per the spec.

## Gates (paste the last lines of each)
- cargo fmt --check: `cargo fmt --all -- --check` -> exit 0
- cargo clippy: `cargo clippy --all-targets -- -D warnings` -> `Finished dev profile`
- cargo test: `cargo test --locked` -> 472 passed, 0 failed (bun present; bun tests skip with a note when it is not)
- filet check: exit 0, no output
- bun builds/tests: no `studio/` or `desktop/` changes; the TS parts are exercised by the Rust integration tests
- no-border grep: not applicable (no UI paths)
- spec gate: `cd examples/mech && fusible body new probe && fusible body probe && fusible check` -> body new and body build succeed; check exits 1 (see Notes), then `git checkout -- . ; git clean` restored the tree

## Not done / blocked
- `Diagnostic` has no `standard` / `source_url` fields yet (track D adds them). Mechanical findings carry `source_url` in `Finding` and append `Source: <url>` to the diagnostic help. After merging D, map `Finding::source_url` to the new field in `Finding::diagnostic()`.
- `mech.component_clearance` measures the finished shape against the board keep-out boxes inside `part.ts` (inflated by the gap, volume from `common`). It does not call the assembly probe machinery, because the assembly script needs a placed body and a full measure run. Same result for keep-outs; the slab and holes are not covered by this rule.
- `mech.standoff_height` needs `board` component heights. Enclosures record no `below`, so it never fires on generated bodies.
- No documentation page for code parts (not in the paths I own).

## Notes for the integrator (conflicts, contract questions)
- Ambiguities I resolved, please confirm:
  1. Part frame: the board frame (X/Y = PCB, Z = 0 at the board mid-plane), because the assembly probes use it. `shell` puts the floor top 5 mm under the board (`STANDOFF = 5`), the inner wall 0.5 mm off the board edge. So a template `boss(..., {h: 5})` ends exactly at the board bottom face.
  2. The spec template (kept verbatim, plus a type annotation on `outline`) has no `cad.cutout`, so on a board with a connector `fusible check` blows with `mech.component_clearance`. `examples/mech/mech/case.ts` cuts the openings and passes.
  3. `board.holes` in `Ctx` holds non-plated holes only (mounting holes).
  4. Injection boss over 2.5 x hole is an error (with a "add gussets" note); boss under 2 x hole is a warning, as the spec says.
  5. `examples/circuit.json` has no `pcb_board`, so `examples/mech/circuit.json` is that circuit plus a board, 4 holes, 3 components with 3D heights and a connector `J1`.
  6. `cable_insertion_center` is looked up on `source_component` and `pcb_component` elements. I did not find where tscircuit emits it.
  7. Hole exemption uses a footprint of any layer, as the addendum words it.
  8. A code part without `board:` follows the only board (`body_board`), but is not required to have bosses for its holes (`fitted_board`), so a loose bracket in an assembly is not flagged by `missing_bosses`.
  9. `format: all` for a code part writes step, stl, glb and brep. `body new` writes `format: all` so the BOM can read the STL.
  10. `/api/bodies` gives `glb: null` for generated bodies (served by `/api/mechanical.glb`).
- Shared-file edits, additive: `src/cli/args.rs` (`BodyAction::New`, `NewArgs`), `src/dispatch/body.rs` (one arm), `src/dev/server.rs` (one arm), `src/config/{types,validate,query,mod}.rs`, `src/commands/check.rs` (`check_manifest_assembly` now also runs `verify_mech`), `src/commands/assembly/{parts,measure,probe,probe_holes,rules,rules_hole}.rs`, `src/dev/{bodies,authored,project,mod}.rs`.
- `probes()` gained a `min_hole` argument and `Role::HoleNear` a field `exempt`.
- Existing test fixtures with an edge connector `J1` needed `"ftype":"simple_pin_header"` after addendum 1 (`tests/body_actions.rs`, assembly tests).
- `src/commands/body/generator.rs`: `run_capture` returns stdout (OpenCascade prints noise on stdout; the report is the last line starting with `{`).
