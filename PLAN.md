# Plan: Fusible 2.0

Written 2026-11-10 against `main` at `583f353` (version 1.1.2). Cold-start handoff: a reader needs nothing but this file, `AGENTS.md`, and the report "Fusible 2.0 research and project review".

Decisions taken by the owner on 2026-11-10:
- Version 2.0, break cleanly. Rule ids, the JSON verdict and the manifest may change, with one migration command.
- Declared electrical intent comes from tscircuit `pinAttributes` and `noConnect` in the TSX. Name heuristics stay only as a warning-level fallback.
- All four areas are in scope: ERC, DFM/DFA, firmware bridge, architecture and agent UX.
- Firmware targets: deeper Rust, C header, Zephyr devicetree, flash and bring-up.

Checked against: `[filet]` (`filet.yml` at the root: 250 lines per file, 8 functions per file, 30 lines per function, complexity 10) and `[muse]` (studio, desktop, mobile and site pin one `@facile/muse` tag). Not applicable and not consulted: migrations, auth/porte, module-path, events. Fusible has no Go API, no database and no login.

## Goal

Make `fusible check` a real design review: rules that work from declared pin roles and typed data, cover fabrication and assembly, and report located, waivable, machine-fixable diagnostics through one pipeline shared by the CLI, the studio and MCP; then carry the verified netlist into firmware for Rust, C and Zephyr.

## Why (evidence)

`src/linter/rules/bypass_cap.rs` decides a pin is a power pin when its name contains `5V` or `VIN`, while tscircuit already writes `requires_power`, `provides_power`, `must_be_connected` and `do_not_connect` on every `source_port`; `src/parser/elements.rs` reads none of them, and 17 files dig through untyped `raw_elements` for everything the six-type parser does not model.

## Approach

Phase 0 rebuilds the base that every rule stands on: a typed Circuit JSON model, one check pipeline, a rule registry with stable ids, located diagnostics, and waivers. It reuses what exists: the `Rule` trait, `LintContext`, the union-find netlist, `parser/geometry.rs`, the YAML fab profiles, and the `RuleInfo` served at `/api/rules`. Phases A to D then run in parallel worktrees on disjoint files, the way `PLAN.md` ran 0.18.0. Phase E integrates and ships. No new runtime dependency is needed before Phase C.

## Steps (ordered)

Every step ends with the four gates of `AGENTS.md` section 3. `[filet]` applies to every Rust and TS step; plan files under the limits, never raise them.

### Phase 0: foundations (one agent, blocks everything else)

1. `src/parser/elements.rs`, new `src/parser/pin.rs` — add `PinAttributes` (serde, every field optional, `deny` nothing) flattened into `Port`; add `do_not_connect`, `must_be_connected`, `provides_power`, `requires_power`, `provides_ground`, `requires_ground`, pull-up, output-mode and `should_have_decoupling_capacitor` fields with the exact snake_case names of `source_pin_attributes.ts`. Exit: a fixture built from `<chip pinAttributes>` round-trips in a unit test.
2. `src/parser/elements.rs`, new `src/parser/props.rs` — extend `Component` with `manufacturer_part_number`, `supplier_part_numbers`, and typed values (`resistance`, `capacitance`, `max_voltage_rating`, `display_value`). First run `rg -o '"type":"source_component"[^}]*' /home/yann/Code/Kono/FieldCam/dist/circuit.json | head` and name fields from real output, not from memory.
3. New `src/parser/model/` (`board.rs`, `pcb.rs`, `silk.rs`, `issues.rs`) — typed structs for `pcb_board`, `pcb_component`, `pcb_trace`, `pcb_via`, `pcb_plated_hole`, `pcb_hole`, `pcb_copper_pour`, `pcb_silkscreen_text`, `pcb_silkscreen_path`, `pcb_courtyard_rect`, `pcb_courtyard_outline`, `source_component_internal_connection`, and a generic `UpstreamIssue { kind, message, element_ids }` for every `*_error` and `*_warning`. `src/parser/geometry.rs` becomes a view over these.
4. Migrate the 9 non-test `raw_elements` readers (`rg -l raw_elements src --glob '!*test*'`) to the typed model, one commit per module (`linter/rules`, `gerber`, `svg`, `commands/body`, `commands/assembly`, `bom`). Then make `raw_elements` `pub(crate)` to the parser only. Exit: `rg raw_elements src` hits only `src/parser/`.
5. `src/netlist/builder.rs`, `src/netlist/joins.rs` — join pins listed in `source_component_internal_connection` into one node. Check first whether this is already done: `rg internal_connection src/netlist`. Add a merge test in `src/netlist/merge_tests.rs`.
6. New `src/project/` (`mod.rs`, `context.rs`) — `Project { root, config, design }` loaded once in `src/main.rs` and passed down. Replace the 20 calls to `FusibleConfig::nearest()` and `DesignRules::nearest()` (`rg 'nearest\(\)' src`). Exit: no `nearest()` call outside `src/project/` and `src/main.rs`.
7. New `src/check/` (`mod.rs`, `report.rs`, `board.rs`, `cross.rs`) — one `check::run(&Project, Scope) -> Report`. `Report { version: 2, fuse, boards[], links[], assembly[], waived[] }`. It returns data and prints nothing. `src/commands/check.rs` renders it with miette; `src/commands/check_json.rs` serialises it; `src/dev/pipeline.rs` and `src/commands/mcp/tools/check.rs` call it. Delete `DevDiagnostic` (the `Report` diagnostic type replaces it) and the second parse in `check_board`. Pin generation moves out of check into `build` (step 31).
8. `src/linter/diagnostics.rs` — add `location: Option<Location { board, x, y, layer }>`, `source: Option<SourceRef { file, line }>` and `fixes: Vec<Fix>` where `Fix` is `SetProp { selector, prop, value }`, `AddElement { tsx }` or `Note(String)`. Keep `help`. Add `Severity::Info`.
9. `src/linter/rule.rs`, `src/linter/rules.rs`, delete `src/linter/summaries.rs` — each rule carries `id`, `category` (`netlist`, `erc`, `drc`, `dfm`, `dfa`, `firmware`, `mechanical`, `upstream`), `summary`, `standard`, `default_severity` and `needs` as one `const META: RuleMeta`. New ids: `decoupling_capacitor` stays, `E_SCHEMATIC_SCALE_EXCESSIVE` becomes `schematic_scale`, `autoroute` splits into `unrouted` plus one id per upstream kind (step 10). Rename `bypass_cap.rs` to `decoupling_capacitor.rs`. A test asserts every id is snake_case, unique, and has a non-empty summary.
10. New `src/linter/rules/upstream.rs` — map each tscircuit `*_error` type to its own rule id (`footprint_overlap`, `off_board`, `via_clearance`, `pad_trace_clearance`, `copper_over_keepout`, ...) and each `*_warning` to a warning. An unknown `*_error` type still blows the fuse as `upstream_error` (fail closed).
11. `src/config/` (new `waivers.rs`, `rules.rs`; edit `raw.rs`, `validate.rs`) — manifest keys:
    ```yaml
    rules:
      test_point_coverage: off      # off | info | warning | error
    waivers:
      - rule: decoupling_capacitor
        match: "U3.VDD"             # element id or refdes.pin, exact
        reason: "bulk cap C12 shared, checked on rev A"
        until: 2027-03-01           # optional
    ```
    A waiver without `reason`, with an unknown rule id, or that matches nothing is a config error (exit 2). An expired waiver stops applying. Waived diagnostics appear in `Report.waived`, never vanish. Netlist-category rules (`shorted_nets`, `ghost_net`, `malformed_element`, `dangling_reference`) cannot be turned off or waived.
12. New `src/commands/migrate.rs`, `src/cli.rs` — `fusible migrate` rewrites a 1.x `fusible.yml` (legacy `hardware:` form, old rule ids) to 2.0 and prints a diff. `--check` exits 1 when a rewrite is needed.
13. Library output — replace `println!`/`eprintln!` outside `src/main.rs`, `src/dispatch/` and `src/commands/*` render functions with returned values or a `Reporter` trait passed in (`rg -c 'println!|eprintln!' src`). Exit: `fusible check --json` and `fusible mcp` write nothing but JSON on stdout, asserted in `tests/exit_code.rs` and `tests/mcp.rs`.

Phase 0 exit: FieldCam (`/home/yann/Code/Kono/FieldCam`) gives the same fuse verdict as 1.1.2 on the old rules; `fusible check --json` there contains links and assembly.

### Phase A: electrical rules (`feat/erc`, owns `src/linter/rules/erc/`, `examples/erc-*.json`)

Each rule is one file, one fixture in `examples/`, one test in `tests/erc_rules.rs`. A rule whose input attribute is absent on every pin of the board reports once at `info` level ("no pin on U1 declares its role") and falls back to the name heuristic at `warning` level; it never passes silently.

14. New `src/linter/rules/erc/roles.rs` — the one place that classifies a pin: `Role { PowerIn, PowerOut, GroundIn, GroundOut, Output, OpenDrain, Input, Bidirectional, Passive, NoConnect, Unknown }` from `PinAttributes`, with the heuristic fallback moved here from `bypass_cap.rs` and `codegen/select.rs` (delete both copies).
15. `erc/pin_conflict.rs` — per net, error on two push-pull outputs, output against power output, two power outputs. Matrix as a `const` table, cells cite the KiCad defaults.
16. `erc/power_undriven.rs` — a net with a `PowerIn` and no `PowerOut`. A connector pin, a battery, or a `nets:` entry with `source: external` counts as a driver (add `source` to `NetSpec` in `src/config/design.rs`).
17. `erc/pin_unconnected.rs` and `erc/nc_connected.rs` — `must_be_connected` pin without a net; `do_not_connect` pin on a net with another endpoint.
18. `erc/missing_pullup.rs` — a net with an open-drain driver or a `needs_external_pullup` pin and no resistor from that net to a rail; same for pull-down. An `is_using_internal_pullup` pin satisfies it.
19. `erc/voltage_domain.rs` — a pin's required voltage against its net's `nets:` voltage with `required_voltage_tolerance`; an output on a higher-voltage domain driving an input on a lower one. Enabled only when the voltages are declared.
20. `erc/rail_budget.rs` and `erc/derating.rs` — sum of `nets:` loads against the declared source limit; capacitor `max_voltage_rating` below 1.25 times the rail voltage (factor in `src/linter/rules/data/derating.yml`, with its source).
21. `decoupling_capacitor.rs` — drive from `should_have_decoupling_capacitor` and `Role::PowerIn`; keep the 3 mm distance check.
22. `erc/led_series_resistor.rs`, `erc/bring_up.rs` — LED on a rail with no series resistance; warnings for a rail without a test point, an MCU without a debug or programming connector, an external connector signal without a clamp. All three default to `warning`.

### Phase B: fabrication and assembly (`feat/dfm`, owns `src/linter/rules/dfm/`, `src/gerber/`, `src/commands/export/`)

23. `src/config/fab.rs`, `src/linter/rules/data/fab/*.yml` — add `min_edge_clearance`, `min_drill_to_copper`, `min_hole_spacing`, `max_aspect_ratio`, `min_mask_dam`, `min_silk_height`, `min_silk_stroke`, `min_silk_to_pad`. Fill each built-in profile from the fab's current capability page and record the page URL and date in the file's `source:`; a missing number leaves the rule disabled for that profile with an `info` line.
24. `dfm/` — one file each: `annular_ring.rs`, `drill_to_copper.rs`, `hole_spacing.rs`, `via_aspect_ratio.rs`, `edge_clearance.rs`, `silk_over_pad.rs`, `silk_text_size.rs`, `mask_sliver.rs`, `acid_trap.rs` (angle under 90 degrees between consecutive segments or at a pad entry; `warning`). Reuse `geom2d.rs` and the `rstar` index already used by `trace_clearance.rs`.
25. `dfa/` — `courtyard_overlap.rs` (rotation-aware; upstream ignores `pcbRotation`, see the wiki page `tools/fusible.md`), `component_edge_clearance.rs`, `fiducials.rs` (only when `assembly_service: true` in the fab block), `bom_incomplete.rs` (placed part without an MPN in the component or `bom.yml`).
26. Done before 2.0: non-plated holes already reach the NPTH drill file (`src/gerber/tests/validation.rs`, `mounting_holes_go_to_npth_drill_file`). Nothing to do.
27. New `src/fab/ipc356.rs`, `src/fab/centroid.rs` — IPC-D-356 netlist and pick-and-place CSV (refdes, x, y, side, rotation; units and rotation convention in the header). `--preset fab` writes both plus a `README.txt` layer map with board name, version and date.
28. `.github/workflows/ci.yml` — extend the gerbv job to render the NPTH file and fail on an empty layer.

### Phase C: firmware bridge (`feat/firmware`, owns `src/silicon/`, `src/codegen/`, `src/commands/flash.rs`)

29. New `scripts/silicon-gen.ts`, `src/silicon/data/` — generate pin tables from the `stm32-data-generated` JSON into compact files embedded with `include_str!`, for the STM32 families G0, G4, F4, L4, H7 first. Read that repo's licence and JSON layout before designing the format (it was not read during research). Keep `rp2040.rs` and `esp32s3.rs` hand-written; add ESP32-C3 and RP2350 the same way. A test compares the generated STM32G431 table with today's `src/silicon/database/stm32/` before that directory is deleted.
30. `src/codegen/` (new `model.rs`, `rust.rs`, `c.rs`, `zephyr.rs`) — one `BoardBindings` model from netlist plus silicon: pins, buses (I2C with the devices on it, SPI with chip selects, UART pairs), ADC inputs with their divider ratio when two resistors form one. Three renderers: `pins.rs` plus `board.rs` (Embassy `init`), `pins.h`, and a Zephyr overlay (`pinctrl` nodes, bus nodes with child devices). Snapshot tests in `src/codegen/tests/`.
31. `src/config/types.rs`, `src/commands/build.rs` — `firmware.<name>.outputs: [rust, c, zephyr]` with paths; `framework` gains `zephyr` and `esp-idf-c`. Generation runs in `build` and `check --write-bindings`, never as a side effect of a plain `check`.
32. New `src/commands/flash.rs`, `src/cli.rs` — `fusible flash [FIRMWARE]`: fuse-gated, then runs the firmware's `flash_cmd`, defaulting to `probe-rs run --chip <chip>` or `espflash flash` by framework. Fusible shells out; it links neither tool. `fusible doctor` reports both.
33. New `src/commands/bringup.rs` — `fusible bringup [BOARD]` writes `dist/bringup.md`: rails in power order with expected voltage and the test point to probe, strap pin states, debug connector pinout. Data comes from `nets:`, roles and the netlist.

### Phase D: agent and human experience (`feat/ux`, owns `src/commands/mcp/`, `src/commands/init/`, `studio/`, `docs/`, `site/`)

34. New `src/commands/explain.rs` — `fusible explain <rule>` and `fusible rules [--json]` from the registry (the MCP `explain_rule` tool calls the same code). `docs/rules/` is generated from it by `scripts/rules-doc.ts`; CI fails when the generated files differ.
35. `src/commands/mcp/tools/` — `check` returns `Report` v2; new `apply_fix` tool applies a `Fix::SetProp` through the existing `edit_board` path; new `waive` tool appends a waiver and requires a reason.
36. New `src/commands/new.rs`, `src/commands/init/templates/` — `fusible new <name> --template esp32-s3|stm32g4|rp2040|blank`: a board with `pinAttributes` set on the MCU and regulator, a firmware crate that builds, a generated enclosure, a passing `fusible check`. Each template is a CI fixture.
37. `studio/src/` `[muse]` — Checks panel: group by category, click a diagnostic to zoom to `location`, show waived items with their reason, an "Apply fix" button for `SetProp` fixes. Rebuild `src/assets/studio.bundle.js`.
38. Docs — delete `HANDOFF.md` and `PLAN.md`, rewrite `ROADMAP.md` for the 2.0 manifest, update `AGENTS.md` sections 1, 2 and 4, `README.md`, `docs/agents.md`, the `fusible` agent skill, `Cargo.toml` description, and both locales of `site/` content. Write `docs/migrate-2.0.md`.

### Phase E: integrate and ship (one agent, after A to D)

39. Workspace split — `crates/fusible-core` (parser, netlist, linter, silicon, check; no IO, no process spawn), `crates/fusible-fab` (gerber, svg, fab, router), root `fusible` binary (commands, dev, tscircuit). Move only; no behaviour change in this commit. Exit: `cargo build -p fusible-core --target wasm32-unknown-unknown` succeeds in CI.
40. `src/dev/` — group the 80 flat files into `git/`, `kicad/`, `assembly/`, `run/`, `bodies/` sub-modules. Move only.
41. Acceptance — FieldCam: add `pinAttributes` to its MCU, modem and regulators in a FieldCam branch; `fusible check` must report the known faults listed in `HANDOFF.md` before its deletion (modem on a 600 mA LDO as `rail_budget`, 1.8 V UART on a 3.3 V MCU as `voltage_domain`). A false positive on FieldCam is a bug in the rule.
42. `CHANGELOG.md` 2.0.0 section with a "Breaking" list, then `/ship`.

## Files to Modify / New

- New: `src/parser/pin.rs`, `src/parser/props.rs`, `src/parser/model/`, `src/project/`, `src/check/`, `src/config/waivers.rs`, `src/config/rules.rs`, `src/linter/rules/{upstream.rs,erc/,dfm/,dfa/}`, `src/fab/`, `src/silicon/data/`, `src/codegen/{model,rust,c,zephyr}.rs`, `src/commands/{migrate,flash,bringup,explain,new}.rs`, `scripts/{silicon-gen,rules-doc}.ts`, `docs/rules/`, `docs/migrate-2.0.md`, `tests/{erc_rules,dfm_rules,waivers,migrate}.rs`.
- Modified: `src/parser/elements.rs`, `src/parser/geometry.rs`, `src/netlist/{builder,joins}.rs`, `src/linter/{rule,rules,diagnostics,info}.rs`, `src/config/{raw,validate,design,fab,types}.rs`, `src/commands/{check,check_json,build}.rs`, `src/dev/pipeline.rs`, `src/commands/mcp/tools/*.rs`, `src/gerber/excellon.rs`, `src/cli.rs`, `src/cli/args.rs`, `src/dispatch/`, `studio/src/`, `.github/workflows/ci.yml`.
- Deleted: `src/linter/summaries.rs`, `src/linter/rules/autoroute.rs` (split), `src/silicon/database/stm32/`, `HANDOFF.md`, `PLAN.md`.

## Exit criteria

- `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test --locked`, and `filet check .` printing nothing, on every merge.
- `rg raw_elements src` hits only the parser; `rg 'nearest\(\)' src` hits only `src/project/` and `src/main.rs`.
- `fusible rules --json` lists at least 40 rules, each with id, category, summary and default severity.
- A board fixture with two push-pull outputs on one net, an undriven `requires_power` pin, and a wired `do_not_connect` pin blows the fuse with three distinct rule ids; adding a reasoned waiver for one moves it to `waived` and the exit code follows the other two.
- `fusible check --json` on FieldCam reports links and assembly, and step 41's two known faults.
- `fusible new demo --template esp32-s3 && cd demo && fusible check && fusible build` exits 0 in CI.
- `fusible migrate --check` exits 0 on every example and template.

## Risks / unknown unknowns

- Field names in step 2 and element shapes in step 3 are taken from circuit-json sources and one real board, not from a schema guarantee. tscircuit moves fast; pin the tested `tscircuit` version in `fusible doctor` and warn outside it.
- Few existing boards set `pinAttributes`. On day one most ERC rules run on the heuristic fallback and produce warnings. The templates (step 36) and FieldCam (step 41) are what make the rules bite; without them 2.0 looks like more noise.
- `stm32-data-generated` size and licence are unread. If embedding is too heavy, ship tables per family behind cargo features or download on `doctor --install`.
- Fab capability numbers change and the JLCPCB page did not extract cleanly. Step 23 needs a person reading the page, not a scrape.
- Step 4 touches Gerber and SVG output. The gerbv CI job and SVG snapshot tests are the guard; run them before and after each module migration.
- Step 39 moves about 500 files. Do it last, in one commit, with no other branch open.
- Zephyr overlay correctness cannot be tested without a Zephyr toolchain. First release marks it experimental and tests only the text snapshot.

## Skip (YAGNI)

- A Fusible parts library. The owner chose `pinAttributes` in TSX; templates carry the examples.
- A constraint solver or automatic part picking in the atopile style. `nets:` declarations and assertions are enough for 2.0.
- SPICE simulation, signal integrity, impedance and length-matching rules. They need a stack-up model that does not exist yet.
- Crystal load capacitor calculation, power sequencing, I2C address conflicts. Each needs per-part data; projects can write them today with tscircuit's `<drccheck>`, which step 10 already surfaces.
- A plugin system for third-party rules. `<drccheck>` covers project rules.
- Panelisation, ODB++, IPC-2581, fab drawings as PDF.
- Arduino and PlatformIO generators. `pins.h` serves both.
- Rewriting the router, the git Sync panel, the desktop updater or the mobile app.
