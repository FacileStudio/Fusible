# Track E: mechanical workspace (code-first parts, materials, mechanical rules)

Read `docs/v1/README.md` first.

Worktree: `/home/yann/Code/Kono/Fusible-v1-mech`, branch `feat/v1-mech`.

## Goal

Go beyond the generated enclosure:
- Mechanical parts are written as TypeScript modules on the OpenCascade kernel Fusible already uses (`occt-wasm` 5.7.1). They are modelled against the real board (outline, holes, component heights, openings).
- Parts are built to STEP, STL and GLB, with reproducible output.
- Parts carry a material and a process.
- Process-aware mechanical rules feed the same fuse.

## Paths you own

- `src/commands/body/` (all of it)
- `src/assets/part.ts` (new), `src/assets/part.d.ts` (new), `src/assets/enclosure.ts` (only to record features, step 5)
- `src/bom/` (materials, mass, cost)
- `src/config/types.rs` (`BodyConfig` fields only)
- `src/cli/args.rs` (`BodyAction::New` only)
- `src/dev/bodies.rs` (add the `GET /api/bodies` list), `src/dev/server.rs` (one arm: `"/api/bodies" => bodies::list(request, state)`)
- `src/commands/assembly/` (only if the mechanical rules plug in there, step 6)
- `examples/mech/` (new), `tests/`

Don't touch `src/linter/` (track D), `studio/` (track A) or `src/dev/pair` (track C).

## Facts about the current code (read first)

- `src/commands/body/mod.rs` and its submodules:
  - Bodies are `Authored` (a `source:` STEP/STL file) or `Generated` (`generate: enclosure`), selected in `jobs.rs` `select_jobs`.
  - `generator.rs` `run_script(script, payload)` runs an embedded TS script under bun with JSON on stdin, in a cache dir that has `occt-wasm` installed (`generator_setup.rs`, `OCCT_WASM_VERSION = "5.7.1"`).
  - `src/assets/enclosure.ts` is the reference for how to call `OcctKernel` (`makeWire`, `makeLineEdge`, `offsetWire2D`, `makeFace`, `extrude`, `translate`, booleans, export).
- `src/commands/body/kit_data.rs` `build_kit(json, board) -> Kit`. That's the board reference: outline, holes, openings, component heights and keep-outs (`kit_levels.rs`). `fusible body kit` already exports it.
- `src/bom/stl.rs` `stl_grams(path)` uses a fixed PLA density (1.24 g/cm³) and `EFFECTIVE_FILL`.
- `src/commands/assembly/rules.rs` has the mechanical fuse rules `body_interference`, `hole_misaligned` and `connector_blocked`, with probes from `src/assets/probes.ts`.
- `src/dev/bodies.rs` serves `GET /api/bodies/<name>.glb` for authored bodies.

Before writing any TS, install `occt-wasm@5.7.1` in your scratch dir and read its type definitions. Use only functions that exist:

```sh
mkdir -p /tmp/claude-1000/occt && cd /tmp/claude-1000/occt && bun init -y >/dev/null && bun add occt-wasm@5.7.1 && ls node_modules/occt-wasm && rg -n "^\s+[a-zA-Z0-9]+\(" node_modules/occt-wasm/dist/*.d.ts | head -200
```

## Steps

### 1. Config (`src/config/types.rs`)

`BodyConfig` gains:

```rust
#[serde(default)] pub material: Option<String>,   // key in materials.yml
#[serde(default)] pub process: Option<String>,    // fdm | sla | mjf | injection | cnc
```

- Validation: `material` must exist in the materials library, and `process` must be one of the five values and listed in that material's `processes`.
- A body with a `.ts` `source` is a **code part**: `Job::Code { name, body, board: Option<Box<BoardTarget>> }`. The board is `body.board` if set; otherwise the first board when there is exactly one.
- `.step`, `.stp` and `.stl` sources stay `Authored`.

### 2. Materials (`src/bom/materials.yml`, embedded with `include_str!`)

```yaml
pla:      { name: PLA,            density: 1.24, cost_per_kg: 20,  max_temp_c: 55,  processes: [fdm] }
petg:     { name: PETG,           density: 1.27, cost_per_kg: 22,  max_temp_c: 70,  processes: [fdm] }
abs:      { name: ABS,            density: 1.04, cost_per_kg: 22,  max_temp_c: 85,  processes: [fdm, injection] }
asa:      { name: ASA,            density: 1.07, cost_per_kg: 28,  max_temp_c: 90,  processes: [fdm, injection] }
pa12:     { name: PA12 (nylon),   density: 1.01, cost_per_kg: 80,  max_temp_c: 120, processes: [mjf] }
resin:    { name: Standard resin, density: 1.18, cost_per_kg: 50,  max_temp_c: 50,  processes: [sla] }
pc:       { name: Polycarbonate,  density: 1.20, cost_per_kg: 35,  max_temp_c: 115, processes: [fdm, injection] }
al6061:   { name: Aluminium 6061, density: 2.70, cost_per_kg: 6,   max_temp_c: 150, processes: [cnc] }
```

- Prices are in EUR, indicative, and flagged `estimated` in the BOM, like the existing part estimates.
- Generalize `stl_grams`: `stl_mass(path, material, process)`. Effective fill is `EFFECTIVE_FILL` for `fdm` and 1.0 for the others.
- With no material, the default stays PLA + FDM (today's behaviour). Existing BOM tests must still pass.
- The BOM `prints` section shows material, process, grams and cost per body.

### 3. Part API (`src/assets/part.ts`, `src/assets/part.d.ts`)

`part.ts` is an embedded runner like `enclosure.ts`.
- **Input on stdin:** `{ source: "<abs path to mech/x.ts>", kit: <Kit JSON or null>, material, process, outputs: [{format, path}] }`.
- It builds a `Part` API object around `OcctKernel` and calls `(await import(pathToFileURL(source).href)).default(ctx)`. The result must be a shape handle.
- It writes the outputs. For STEP, **fix the timestamp**: post-process the header `FILE_NAME(...)` date to `'2000-01-01T00:00:00'`, so rebuilds are byte-identical (Pastille commit 885e536).
- It prints one JSON line to stdout: `{"features":[...],"bbox":[x,y,z],"volume_mm3":N}`.

`ctx` shape (also written to `part.d.ts`, which `fusible body new` copies to `mech/fusible.d.ts` for editor types):

```ts
export type Ctx = {
  board: null | { outline: [number, number][]; thickness: number;
                  holes: { x: number; y: number; d: number }[];
                  openings: { name: string; x: number; y: number; z: number; w: number; h: number; side: string }[];
                  components: { name: string; x: number; y: number; top: number; bottom: number }[] };
  material: string; process: "fdm" | "sla" | "mjf" | "injection" | "cnc";
  cad: Cad;
};
export type Shape = { readonly __shape: unique symbol };
export type Cad = {
  box(w: number, d: number, h: number): Shape;
  cylinder(r: number, h: number): Shape;
  extrude(points: [number, number][], h: number): Shape;
  union(...s: Shape[]): Shape; cut(a: Shape, ...b: Shape[]): Shape; intersect(a: Shape, b: Shape): Shape;
  move(s: Shape, x: number, y: number, z: number): Shape; rotate(s: Shape, axis: "x" | "y" | "z", deg: number): Shape;
  fillet(s: Shape, r: number): Shape;        // only if occt-wasm exposes a fillet; else omit from API and d.ts
  shell(outline: [number, number][], wall: number, h: number, floor: number): Shape; // recorded feature "wall"
  boss(x: number, y: number, opts: { insert?: "m2" | "m2.5" | "m3"; screw?: "m2" | "m2.5" | "m3"; h: number }): Shape; // feature "boss"
  standoff(x: number, y: number, h: number, hole: number): Shape;  // feature "standoff"
  ventSlots(x: number, y: number, n: number, w: number, l: number, pitch: number, dir: "x" | "y"): Shape; // feature "vent"
  cutout(opening: { x: number; y: number; z: number; w: number; h: number; side: string }, margin?: number): Shape; // feature "port"
  draft(deg: number): void;                  // records the declared draft for the part (feature "draft")
};
```

- Fixed hardware sizes, as constants in `part.ts`:

| Size | Heat-set insert bore | Insert length | Boss outer diameter | Clearance hole | Self-tap pilot |
|---|---|---|---|---|---|
| m2 | 3.2 mm | 4 mm | 6.0 mm | 2.4 mm | 1.6 mm |
| m2.5 | 3.6 mm | 5 mm | 7.0 mm | 2.9 mm | 2.1 mm |
| m3 | 4.0 mm | 5.7 mm | 8.0 mm | 3.4 mm | 2.5 mm |

  - The insert bore and length values are typical brass heat-set insert sizes. They are not from the research sources, so expose them as `INSERTS` in `part.ts`, and make every message that uses them say "typical insert size, check your insert's datasheet".
  - Boss outer diameter ≈ 2× the insert, per https://www.dfamacademy.com/guides/enclosure-design-3d-printing.
  - Clearance hole = +0.25 mm on the diameter and self-tap pilot = -0.25 mm (both per https://www.hubs.com/knowledge-base/enclosure-design-3d-printing-step-step-guide/). Screw nominal sizes are 2, 2.5 and 3.
- Every helper that records a feature pushes `{kind, ...params, at:[x,y,z]}` to `features`.
- Raw `box` / `extrude` / `cut` record nothing. The mechanical rules only check recorded features plus the geometric probes (step 6). Write that in `part.d.ts` docs: "use the helpers to get rule coverage".

### 4. `fusible body new <name>`

- `BodyAction::New(NewArgs { name: String, #[arg(long)] board: Option<String>, #[arg(long, default_value = "petg")] material: String, #[arg(long, default_value = "fdm")] process: String })`.
- **It writes:**
  - `mech/<name>.ts` (template below);
  - `mech/fusible.d.ts` (a copy of `part.d.ts`, overwritten on every run);
  - a `bodies.<name>` entry in `fusible.yml`, through the existing YAML edit helpers in `eject_manifest.rs` / `yaml_lines.rs` (reuse them, don't write a new YAML writer).
- Refuse if the name exists in any of boards, firmware or bodies (names are unique across all three). Exit 2.
- **Template:**

  ```ts
  import type { Ctx } from "./fusible";

  export default ({ board, cad }: Ctx) => {
    const outline = board?.outline ?? [[0, 0], [60, 0], [60, 40], [0, 40]];
    const base = cad.shell(outline, 2, 18, 2);
    const posts = (board?.holes ?? []).map((h) => cad.boss(h.x, h.y, { insert: "m3", h: 5 }));
    return cad.union(base, ...posts);
  };
  ```

- `fusible body <name>` builds code parts through `run_script(PART_TS, payload)`. Output goes to `<out_dir or dist/bodies>/<name>.{step,stl,glb}`, with format defaults like the other bodies.

### 5. Record enclosure features

- `src/assets/enclosure.ts` already knows `wall`, `bosses`, `vents` and `clearance`.
- Make it print the same `{"features":[...]}` JSON line (wall, boss with `outer`/`pilot`, vent `w`, component clearance), so the mechanical rules cover generated enclosures too.
- Keep its outputs byte-identical otherwise.

### 6. Mechanical rules

New file `src/commands/body/mech_rules.rs`, plus `mech_rules_data.rs` for the tables.

- Same diagnostic shape as `src/commands/assembly/rules.rs`. Use its `Diagnostic` type and fuse wiring. Read how `body_interference` reaches the fuse and exit code 1, and register the new rules the same way.
- Every message ends with `"Design guideline, not a certification."`. `source_url` is set.

| Rule id | Check | Limits by process | Source |
|---|---|---|---|
| `mech.wall_thickness` | every `wall` feature | fdm ≥ 2.0 mm, sla ≥ 1.5, mjf ≥ 1.5, injection 1.0–5.0 (error above 5: "thick walls sink"), cnc ≥ 1.0 | dfamacademy (fdm 2–3, sla/mjf 1.5–2), rtpcompany (injection < 5 mm) |
| `mech.boss` | `boss` features | insert bore matches the table; outer ≥ 2 × insert bore (warning otherwise); injection: outer ≤ 2.5 × hole and a note to add gussets | dfamacademy, rtpcompany |
| `mech.port_clearance` | `port` features | margin ≥ 1.0 mm per side on fdm, ≥ 0.5 on sla/mjf | hubs (2 mm total) |
| `mech.vent_width` | `vent` features | fdm w ≥ 1.0 mm, sla ≥ 0.8, mjf ≥ 0.8 | dfamacademy |
| `mech.standoff_height` | `standoff` or `boss` features, when `board` exists | `h` ≥ max component `bottom` height + 0.5 mm. Error otherwise: "U3 sits 2.1 mm under the board; standoffs are 1.5 mm" | dfamacademy (3–5 mm, clear bottom parts) |
| `mech.component_clearance` | geometric: reuse the assembly probe/interference machinery between the part and the board keep-outs | gap ≥ 0.5 mm on fdm, 0.2 on others | hubs (0.5 mm around internal parts) |
| `mech.draft` | injection only | a `draft` feature ≥ 0.5° must exist, otherwise error "declare cad.draft(deg) for molded parts" | rtpcompany (draft on walls, bosses, ribs) |

Source URLs:
- dfamacademy = https://www.dfamacademy.com/guides/enclosure-design-3d-printing
- hubs = https://www.hubs.com/knowledge-base/enclosure-design-3d-printing-step-step-guide/
- rtpcompany = https://www.rtpcompany.com/wp-content/uploads/Part-Design-Guidelines-Brochure.pdf

`fusible check` runs these after the boards, wherever `assembly::verify` already runs (`src/commands/check.rs` near line 140), for bodies whose features are cached from the last build. When a body was never built, emit a warning: `"bracket was not built yet; run fusible body bracket to check it"`.

### 7. `GET /api/bodies` (`src/dev/bodies.rs`)

- Returns `[{"name","kind":"authored"|"generated"|"code","source","material","process","mass_g","cost","bbox","glb":"/api/bodies/<name>.glb?rev=<hash>"}]`.
- `rev` is a hash of the source file's bytes plus mtime. `mass_g`, `cost` and `bbox` are null until built.
- Code parts also serve `/api/bodies/<name>.glb`: build on demand with the existing cache logic (`authored_cache.rs`), keyed by `rev`.

### 8. Examples and tests

- `examples/mech/fusible.yml`: one board (reuse `examples/circuit.json` as `circuit_json`), plus bodies `case` (code part, petg/fdm) and `bracket` (code part, al6061/cnc).
- `examples/mech/mech/case.ts` and `bracket.ts`: use the helpers.
- `examples/mech/mech/bad.ts`: walls of 1.2 mm on fdm and vents of 0.6 mm, so it blows.
- **Rust unit tests** on `mech_rules.rs`, one per rule, with hand-built feature lists. No bun needed.
- **Integration test** (`tests/mech.rs`): runs `fusible body case` and `fusible check` in `examples/mech`. Gate it on bun being available: skip with a printed note when `bun_path()` is None, following how existing body tests guard it (`rg -n "bun_path\(\)" tests src/commands/body/*tests*`).
- **Reproducibility test:** build `case` twice and assert identical STEP bytes.

## Gates

```sh
cargo fmt --all -- --check && cargo clippy --all-targets -- -D warnings && cargo test --locked
filet check
cd examples/mech && cargo run -q -- body new probe && cargo run -q -- body probe && cargo run -q -- check; echo "exit=$?"; git checkout -- . ; git clean -fd mech/probe.ts; cd ../..
```

## Exit criteria

- `fusible body new` and `fusible body <code part>` produce STEP, STL and GLB.
- Mass and cost appear in `fusible bom`.
- Every mechanical rule blows on `bad.ts` or on its unit test.
- `/api/bodies` serves the list, and builds are reproducible.
- Report written to `docs/v1/reports/E.md`, listing which `occt-wasm` functions back each `Cad` method and any you had to omit.

## Addendum from Pastille (kit and assembly false positives)

These came from the Pastille v0.4 build. Fix them in this track; they live in your paths.

1. **Kit openings are too broad.** `src/commands/body/kit_data.rs` (`component_entry`, around line 148) makes a wall opening for any part within 0.5 mm of the board edge (buttons, ICs, 0402s). Restrict openings to components whose kind is a connector (`kind` contains `connector`, `usb`, `jack`, `header`, `socket`) or that declare `cable_insertion_center`. Other edge parts stay in `components` with no opening. `connector_blocked` then fires only for real connectors. Add a test with an edge-mounted 0402 (no opening) and a USB-C (opening).
2. **USB-C keep-out height.** For the SHOU HAN USB-C (LCSC C2765186) the kit `keep_out` (z -1.28..2.88) disagrees with the real part (shell 3.16 mm on the top face, plug axis about board top + 1.58). Add a small override table in `kit_levels.rs`: `("C2765186", top: 3.16)`. Use the part's LCSC id if the circuit JSON carries it (`supplier_part_numbers.lcsc` or `manufacturer_part_number`; check with `rg`), else skip silently. Document the table as "verified part heights". Test it.
3. **Assembly hole rule false positives.** `src/commands/assembly/rules_hole.rs` `missing_bosses` demands top-side material within 4 mm of every non-plated hole when an authored body sets `board:`. That produced 42 false violations in Pastille (mic acoustic port, USB-C pegs, holes under parts). Change it: skip a hole when a component footprint (kit `components`) covers the hole center (hole under a part), and skip holes whose diameter is below 1.0 mm (acoustic ports). Make this configurable: `assembly.hole_min_diameter` (default 1.0) in the manifest. Tests: a hole under a component is not reported; a bare 3.2 mm hole is.
4. **Slim GLB.** The assembly GLB keeps one primitive per face (11,891 primitives, 7 MB). After building a GLB in `convert.rs`/the assembly export, merge primitives that share a material per mesh (a post-processing pass on the glTF JSON and binary buffer; Pastille's reference is `/home/yann/Code/Facile/Pastille/hardware/bodies/slim.ts`: port its logic into `src/assets/convert.ts`). Keep geometry identical; add a test on a small model that the primitive count drops.
