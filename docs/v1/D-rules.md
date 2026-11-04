# Track D: physics and fab rules, `check --json`

Read `docs/v1/README.md` first.

Worktree: `/home/yann/Code/Kono/Fusible-v1-rules`, branch `feat/v1-rules`.

## Goal

Add rules that check physics and fabrication, not just topology:
- `trace_current` (IPC-2152)
- `voltage_clearance` (IPC-2221B)
- `fab_profile` (fab-house capability)
- a stricter `bypass_cap` distance check

Every diagnostic cites its standard. Add `fusible check --json` and `GET /api/rules`.

Every rule message and help text says it is a design pre-check, not certification.

## Paths you own

- `src/linter.rs`, `src/linter/` (all of it), `src/linter/rules/data/` (new)
- `src/parser/` (add a geometry reader; don't change existing structs' serde behaviour)
- `src/config/` (add fields only)
- `src/commands/check.rs`, `src/cli.rs` (`--json` on `Check` only), `src/main.rs` (only if JSON mode needs it)
- `src/dev/pipeline.rs` (pass design rules into the context, extend `DevDiagnostic`)
- `src/dev/firmware.rs` (only the `LintContext::new` call)
- `src/dev/server.rs`: one arm, `"/api/rules" => rules_api::serve(request)`
- `src/dev/rules_api.rs` (new)
- `examples/`, `tests/`

Don't touch `studio/`, `desktop/`, `mobile/`, `site/` or `src/commands/body`.

## Facts about the current code (read these files fully first)

- `src/linter.rs`: the `Rule` trait (`name`, `severity`, `check(&LintContext) -> Vec<Diagnostic>`), and `Linter::default()` registers 9 rules.
- `src/linter/diagnostics.rs`: `Diagnostic { rule, message, help, severity }`, `#[derive(Default)]`, with miette rendering by hand.
- `src/linter/context.rs`: `LintContext { circuit, netlist, mcu }`. It is built in `src/commands/check.rs:67` and `:164`, `src/dev/pipeline.rs:103` and `src/dev/firmware.rs:97`.
- `src/linter/rules/trace_clearance.rs`:
  - Checks pad-to-pad clearance with a fixed `MIN_CLEARANCE_MM = 0.15`.
  - It doesn't read `pcb_trace` geometry yet.
  - `Circuit.raw_elements: Vec<serde_json::Value>` holds every element, so read geometry from there.
- `src/linter/rules/bypass_cap.rs`: the existing decoupling rule. Read it and extend it. Don't duplicate it.
- `src/dev/pipeline.rs`: `DevDiagnostic { rule, message, help }` is what the studio receives.

## Circuit JSON shapes you will read (tscircuit)

```jsonc
{"type":"pcb_board","width":40,"height":30,"thickness":1.6,"num_layers":2,"outline":[{"x":..,"y":..}]}
{"type":"pcb_trace","pcb_trace_id":"pcb_trace_0","source_trace_id":"source_trace_0",
 "route":[{"route_type":"wire","x":1,"y":2,"width":0.15,"layer":"top"},
          {"route_type":"via","x":3,"y":2,"from_layer":"top","to_layer":"bottom"}]}
{"type":"pcb_via","pcb_via_id":"..","x":3,"y":2,"outer_diameter":0.6,"hole_diameter":0.3,"layers":["top","bottom"]}
{"type":"pcb_plated_hole","shape":"circle","x":..,"y":..,"hole_diameter":1.0,"outer_diameter":1.6}
{"type":"pcb_plated_hole","shape":"pill","hole_width":..,"hole_height":..,"outer_width":..,"outer_height":..}
{"type":"pcb_hole","hole_diameter":3.2,"x":..,"y":..}
{"type":"source_trace","source_trace_id":"source_trace_0","connected_source_net_ids":["source_net_1"],"connected_source_port_ids":[..]}
{"type":"source_net","source_net_id":"source_net_1","name":"VBUS"}
```

- Units are mm. Width is per route point, so a segment's width is the width of its start point.
- Run `rg -o '"type":"[a-z_]+"' examples/circuit.json | sort | uniq -c` to see which types the examples actually carry.
- If a field is missing, the element can't be verified. Emit a `malformed_element`-style diagnostic from your rule. Never skip it silently (the fail-closed contract in `src/linter.rs` docs).

## Steps

### 1. Geometry reader (`src/parser/geometry.rs`, new)

- `pub struct Segment { pub trace_id: String, pub net_names: Vec<String>, pub layer: String, pub a: (f64,f64), pub b: (f64,f64), pub width: f64 }`
- `pub struct Via { pub x: f64, pub y: f64, pub outer: f64, pub hole: f64 }`
- `pub struct Hole { pub x: f64, pub y: f64, pub hole: f64, pub outer: Option<f64> }` (plated and non-plated)
- `pub struct Board { pub width: f64, pub height: f64, pub thickness: f64, pub layers: u8 }`
- `pub fn read(circuit: &Circuit) -> Geometry`: one pass over `raw_elements`.
  - It resolves `pcb_trace.source_trace_id` to `source_trace.connected_source_net_ids` to `source_net.name`.
  - It returns `Geometry { segments, vias, holes, board: Option<Board>, unreadable: Vec<String> }`.
- Unit tests on a hand-written JSON fixture.

### 2. Design rules from the manifest (`src/config/`)

Add to `FusibleConfig` (all optional, `#[serde(default)]`):

```rust
pub nets: BTreeMap<String, NetSpec>,          // NetSpec { voltage: Option<f64>, current: Option<f64>, max_rise: Option<f64> }
pub fab: Option<FabSpec>,                     // untagged: Named(String) | Custom(FabProfile)
pub clearance_class: Option<String>,          // "B1" | "B2" | "B4"
pub copper: Option<CopperSpec>,               // CopperSpec { outer_oz: f64 (default 1.0), inner_oz: f64 (default 0.5) }
```

- **Validation** (in `validate.rs`, returning `ConfigError` with a clear message):
  - `clearance_class` must be one of B1, B2, B4. Others get "B3, A5, A6, A7 are not supported yet; use B1, B2 or B4".
  - A named fab must exist in `data/fab/`.
  - Voltages, currents and `max_rise` must be finite and greater than 0.
- Add `pub struct DesignRules { nets, fab: Option<FabProfile>, clearance_class, copper }` built from the config, defaults applied.
- **Don't add a `safety:` key.** Creepage (IEC 62368-1) is out of v1: its tables weren't verified from a primary source.
- Update `src/config/template.rs` so `fusible init` writes a commented-out example of `nets:`, `fab:` and `clearance_class:`. The template is YAML, so YAML comments are allowed.

### 3. `LintContext`

- Add `pub design: Option<&'a DesignRules>` and `with_design(...)`.
- Thread it in at all four construction sites. The board's manifest is reachable from `BoardTarget`; follow how `mcu` gets there. `src/dev/pipeline.rs` gets it from the dev state's manifest (`rg -n "manifest|FusibleConfig" src/dev/state.rs src/dev/pipeline.rs`).
- When there is no manifest, pass `None`. Rules that need declared data then emit nothing. That's allowed: no declaration means no claim to check.

### 4. Diagnostic fields

- `Diagnostic` gains `pub standard: Option<&'static str>`, `pub source_url: Option<&'static str>`, `pub elements: Vec<String>`, `pub fix: Option<String>`.
- Render `standard` in miette's `url()` (`source_url`).
- `DevDiagnostic` gains the same fields, serialized with `skip_serializing_if` for `None` or empty, plus `severity: "error" | "warning"`.

### 5. Rules

Each rule lives in its own file under `src/linter/rules/`. Tests go in `src/linter/rules/tests/<rule>.rs`.

#### 5.1 `trace_current` (error)

- **Source:** https://docs.kicad.org/doxygen/tracks__width__versus__current__formula_8h.html (Brooks and Adam fit to IPC-2152).
- **Formula:** ΔT = K · I^a · W^b · H^c, with W = width in mils, H = copper thickness in mils (1 oz = 1.378 mil, so `oz * 1.378`), and I in A.
- **Constants:**

| Layer | K | a | b | c |
|---|---|---|---|---|
| External (top/bottom), all weights | 215.3 | 2 | -1.15 | -1.0 |
| Internal 0.5 oz | 120 | 2 | -1.10 | -1.52 |
| Internal 1 oz | 200 | 1.9 | -1.10 | -1.52 |
| Internal 2 oz | 300 | 2 | -1.15 | -1.52 |
| Internal 3 oz | 262.5 | 1.9 | -1.15 | -1.52 |

  For internal layers, use the nearest listed weight.
- **Logic:**
  - For each net in `design.nets` with `current`, find its narrowest segment and compute ΔT.
  - Blow the fuse if ΔT > `max_rise` (default 10 °C).
  - Also solve for the minimum width (bisection on W, 0.01 to 20 mm) and put it in `fix`: `"widen VBUS to at least 0.82 mm"`.
- **Message:** `"VBUS carries 2.0 A; its narrowest trace (0.25 mm, top) heats about 34 °C over ambient, above the 10 °C limit"`.
- **Help:** `"IPC-2152 estimate (Brooks and Adam fit). Design pre-check, not certification."`
- `standard: "IPC-2152"`. `elements` are the trace ids.
- If a declared net has no segments: warning `"VBUS declares 2 A but has no routed traces to check"`.
- **Tests:**
  - external 1 oz, 1 A, 0.254 mm (10 mil): compute by hand in the test with the formula and assert within 0.1 °C;
  - an internal-layer case;
  - a passing wide trace;
  - the fix width round-trips (ΔT at the fix width ≤ `max_rise`).

#### 5.2 `voltage_clearance` (error)

- **Source:** IPC-2221B Table 6-1, values cross-checked on https://www.atlaspcb.com/tools/conductor-spacing-calculator and https://pcbtools.xyz/tools/conductor-spacing (both agree).
- Put the table in `src/linter/rules/data/ipc2221b.yml`, loaded with `include_str!` + serde_yaml:

| V (DC or AC peak) | B1 internal | B2 external uncoated | B4 external coated |
|---|---|---|---|
| 0–15 | 0.05 | 0.10 | 0.05 |
| 16–30 | 0.05 | 0.10 | 0.05 |
| 31–50 | 0.10 | 0.60 | 0.13 |
| 51–100 | 0.10 | 0.60 | 0.13 |
| 101–150 | 0.20 | 0.60 | 0.40 |
| 151–170 | 0.20 | 1.25 | 0.40 |
| 171–250 | 0.20 | 1.25 | 0.40 |
| 251–300 | 0.20 | 1.25 | 0.40 |
| 301–500 | 0.25 | 2.50 | 0.80 |
| > 500 | 0.25 + 0.0025/V over 500 | 2.50 + 0.005/V over 500 | 0.80 + 0.00305/V over 500 |

- **Logic:**
  - The voltage difference between two nets is `|V_a - V_b|`. A net with no declared voltage counts as 0 V only if it is named `GND`, `AGND`, `DGND`, `PGND` or `VSS`. Other undeclared nets are skipped.
  - External layers (`top`, `bottom`) use the manifest class (default B2). Inner layers always use B1.
  - Build an `rstar` R-tree (already a dependency) of segment and pad bounding boxes inflated by half their width.
  - Query the neighbours on the same layer and compute the exact segment-to-segment distance minus half-widths (and segment to pad box).
  - Report each net pair once, with the worst distance.
- **Message:** `"MAINS_L (230 V) and GND are 0.80 mm apart on top; IPC-2221B B2 asks at least 1.25 mm"`.
- **Help:** `"IPC-2221B Table 6-1 clearance. Design pre-check, not certification. Safety-rated products also need IEC 62368-1 creepage, which Fusible does not check yet."`
- **Tests:** a 230 V vs GND pair at 0.8 mm blows; at 1.3 mm passes; the B4 class passes at 0.5 mm; the internal layer uses B1; the >500 V formula is checked.

#### 5.3 `fab_profile` (error)

Data files in `src/linter/rules/data/fab/`, each with `name`, `source`, `checked: 2026-11-04`:

`jlcpcb-2l.yml` (source https://www.schemalyzer.com/en/blog/manufacturing/jlcpcb/jlcpcb-design-rules and https://jlcpcb.com/capabilities/pcb-capabilities):

```yaml
name: jlcpcb-2l
source: https://jlcpcb.com/capabilities/pcb-capabilities
checked: 2026-11-04
min_trace: 0.127
min_space: 0.127
min_drill: 0.3
min_via_pad: 0.6
min_annular: 0.15
min_via_to_via: 0.254
drill_step: 0.05
board_min: [5, 5]
board_max: [400, 500]
```

`jlcpcb-4l.yml`: same but `min_trace: 0.09`, `min_space: 0.09`, `min_drill: 0.2`, `min_via_pad: 0.45`, `min_via_to_via: 0.127`.

- **Don't write a `pcbway-std` profile.** It wasn't verified. Custom profiles cover other fabs.
- **Checks:**

| Check | Condition |
|---|---|
| Trace width | `width < min_trace` |
| Same-layer distinct-net spacing | `< min_space` (when a fab is set, this replaces `trace_clearance`'s fixed 0.15) |
| Via drill and hole drill | `< min_drill`, or not a multiple of `drill_step` (tolerance 1e-6) |
| Via pad | `< min_via_pad` |
| Annular ring | `(outer - hole) / 2 < min_annular` |
| Via to via (edge to edge) | `< min_via_to_via` |
| Board size | outside `board_min` / `board_max` |

- Each violation names the profile: `"trace 0.10 mm on top is below jlcpcb-2l minimum 0.127 mm"`. `standard: "jlcpcb-2l"`, `source_url` from the profile.
- `TraceClearanceRule` keeps working unchanged when no `fab` is set.

#### 5.4 `bypass_cap` distance

- Read `bypass_cap.rs`. Add: when the circuit has PCB placement, every decoupling capacitor the rule already pairs with an IC power pin must sit within 3.0 mm (pad edge to pad edge). Otherwise emit a **warning**:
  - `"C3 decouples U1 pin 4 (VDD) from 6.2 mm away; place it within 3 mm"`
  - help `"Shorter loop, lower inductance. Source: https://www.protoexpress.com/blog/decoupling-capacitor-placement-guidelines-pcb-design/"`
- `standard: "layout guideline"`.

### 6. `fusible check --json`

- `Check { ..., #[arg(long)] json: bool }`.
- In JSON mode:
  - print exactly one JSON document to stdout: `{"fuse":"intact"|"blown","boards":[{"name":"main","diagnostics":[<DevDiagnostic-shaped objects>]}]}`;
  - send no emoji lines and no miette output to stdout (progress goes to stderr or is silenced);
  - keep exit codes 0, 1, 2. On a tool error, print `{"fuse":"error","error":"<message>"}` and exit 2.
- Tests in `tests/` with `assert_cmd`-style `std::process::Command` (follow `tests/exit_code.rs`): an intact example gives 0 plus valid JSON; `examples/circuit-broken.json` gives 1 plus `"fuse":"blown"`; a missing file gives 2.

### 7. `GET /api/rules` (`src/dev/rules_api.rs`)

- Returns the registered rules as `[{"id","standard","source_url","severity","enabled","summary"}]`.
- Add `fn standard(&self) -> Option<&'static str> { None }`, `fn source_url(&self) -> Option<&'static str> { None }` and `fn summary(&self) -> &'static str` to the `Rule` trait. Write a one-sentence summary for all 13 rules (the 9 existing plus 4 new).
- Add `Linter::describe() -> Vec<RuleInfo>`.
- `enabled` is false for rules whose data isn't declared: `trace_current` without nets with `current`, `voltage_clearance` without voltages, `fab_profile` without `fab`.

### 8. Examples

- In `examples/`, add one Circuit JSON per new rule that blows it, plus a `fusible.yml` fixture in `tests/fixtures/rules/` that declares the nets and fab. Name them `examples/circuit-trace-current.json`, `examples/circuit-voltage-clearance.json` and `examples/circuit-fab-profile.json`.
- Keep the existing examples passing as before.

## Gates

```sh
cargo fmt --all -- --check && cargo clippy --all-targets -- -D warnings && cargo test --locked
filet check
cargo run -- check examples/circuit.json --json | head -c 400; echo; echo "exit=$?"
```

## Exit criteria

- 4 new or extended rules, each with a blowing example and unit tests.
- `--json` output matches the contract in `docs/v1/README.md`, and `/api/rules` serves the list.
- No change to the existing exit codes or existing example outcomes.
- Report written to `docs/v1/reports/D.md`.

## Addendum: coordination with track I

Track I adds the `autoroute` rule and edits `src/linter.rs` `Linter::default()` with one `.with_rule(...)` line. Keep your registration lines separate to merge cleanly. After merge, `fab_profile` replaces `trace_clearance`'s fixed 0.15 mm only when `fab` is set. Track I's gerber pour clearance reads `fab.min_space` when present; you expose it through `DesignRules` (`design.fab.as_ref().map(|f| f.min_space)`), nothing more.
