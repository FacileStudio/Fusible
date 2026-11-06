<p align="center">
  <img src="assets/logo.svg" alt="Fusible logo" width="72" height="72">
</p>

<h1 align="center">Fusible</h1>

<p align="center">
  <strong>One project for your boards, your firmware, and the case around them.</strong><br>
  Fusible checks the whole design against electrical, physical and mechanical rules.
  When a rule fails, <strong>the fuse blows</strong>: nothing gets exported, generated or flashed.
</p>

<p align="center">
  <a href="https://fusible.facile.studio">Website</a> ·
  <a href="https://fusible.facile.studio/docs">Docs</a> ·
  <a href="https://fusible.facile.studio/download">Download</a> ·
  <a href="https://crates.io/crates/fusible">crates.io</a>
</p>

<p align="center">
  <a href="https://crates.io/crates/fusible"><img src="https://img.shields.io/crates/v/fusible?color=111111&label=crates.io" alt="crates.io version"></a>
  <a href="https://github.com/FacileStudio/Fusible/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/FacileStudio/Fusible/ci.yml?branch=main&color=111111&label=ci" alt="CI status"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-111111" alt="MIT license"></a>
</p>

Fusible reads the Circuit JSON that [tscircuit](https://tscircuit.com) produces,
checks the electrical, physical and mechanical rules of every board and part, and
only then writes fabrication files, firmware pin bindings and enclosures. It ships
as a command line tool, a browser studio (`fusible studio`), a desktop app, a
read-only Android viewer, and an MCP server that lets agents work on the same
project through the same checks.

![The Fusible studio showing a round PCB](assets/screenshots/studio-pcb.png)

<table>
  <tr>
    <td width="50%"><img src="assets/screenshots/studio-3d.png" alt="The studio showing the board in 3D"></td>
    <td width="50%"><img src="assets/screenshots/studio-checks.png" alt="The studio listing checks with their standards"></td>
  </tr>
  <tr>
    <td width="50%"><img src="assets/screenshots/studio-bodies.png" alt="The studio showing a generated enclosure"></td>
    <td width="50%"><img src="assets/screenshots/desktop.png" alt="The desktop app listing projects"></td>
  </tr>
</table>

<p align="center">
  <img src="assets/screenshots/mobile.png" alt="The Android app showing a board summary" width="240">
  <img src="assets/screenshots/mobile-board.png" alt="The Android app showing a board" width="240">
</p>

## What you get

| Piece | What it does |
|---|---|
| Linter | Checks nets, shorts, clearance, decoupling, pin muxing, routing and cabled connectors. |
| Physics rules | Trace current (IPC-2152), voltage clearance (IPC-2221B) and fab capability (JLCPCB), each citing its source. |
| Studio | Live PCB, schematic, 3D, parts, firmware pins, bodies and checks in the browser. |
| Mechanical | Parts written in TypeScript on OpenCascade, with materials, processes and mechanical rules. |
| Firmware | Typed Rust pin bindings (`pins.rs`) for STM32, RP2040 and ESP32-S3. |
| BOM | Prices, picks, material mass and print cost. |
| Fabrication | Gerbers with copper pours, drill files, BOM, pick and place, KiCad, STEP, GLB, SVG, PDF. All behind the fuse. |
| Release | `fusible release` refuses to write fab files from a blown or unrouted board. |
| Agents | `fusible mcp` serves checks and fuse-gated edits to Bulle, Claude Code, Codex and others. |
| Desktop | A signed-update app for Windows, macOS and Linux with first-run onboarding. |
| Mobile | A read-only Android viewer, paired with a QR code on your local network. |
| Suite | Opus tasks from blown fuses, Nuage sharing of fab files. |
| Team | A `SYNC` panel that shares the project through git, with no terminal. |
| KiCad | The studio watches your `.kicad_pcb` and writes moved parts back into your TSX. |

## Install

```sh
cargo install fusible
```

Linting Circuit JSON, Gerbers, and SVG need nothing else. To compile `.tsx`
boards or export STEP, install tscircuit and check the toolchain:

```sh
bun add -g tscircuit
fusible doctor            # reports bun, node, and tsci
fusible doctor --install  # installs what is missing
```

The desktop app (Windows, macOS, Linux) and the Android app are attached to each
[GitHub release](https://github.com/FacileStudio/Fusible/releases/latest). The desktop
app bundles `bun`, lists your projects and runs `fusible studio --desktop` for each.
Installers are not code-signed with Apple or Microsoft certificates, so your OS
will warn you. In-app updates are verified against Fusible's own signing key.

## Quick start

```sh
fusible init --name sensor-node
fusible check
fusible studio
```

`init` writes a `fusible.yml`:

```yaml
name: sensor-node

boards:
  main:
    entry: hardware/index.tsx
    mcu: stm32g431

firmware:
  app:
    board: main
    path: firmware
    framework: embassy
    pins_rs: firmware/src/pins.rs
    build_cmd: cargo build
```

`fusible init --currency USD` also writes a `bom.yml`.

## The project file

A project has three kinds of named things. Names are unique across all three, so
any command takes one as its target.

```yaml
name: rover

boards:
  main:   { entry: boards/main/index.tsx, mcu: esp32-s3 }
  sensor: { entry: boards/sensor/index.tsx, mcu: rp2040, kicad: boards/sensor/sensor.kicad_pcb }

firmware:
  main-fw:   { board: main, path: firmware/main, pins_rs: firmware/main/src/pins.rs }
  sensor-fw: { board: sensor, path: firmware/sensor, pins_rs: firmware/sensor/src/pins.rs }

bodies:
  case: { source: bodies/case.step }
  knob: { source: bodies/knob.step, build: python bodies/knob.py }
  lid:  { generate: enclosure, board: sensor }

links:
  cable:
    from: main.J2
    to: sensor.J1
    wires:
      - { a: 1, b: 1, net: GND }
      - { a: 2, b: 3, nets: [TX, RX] }
```

- `fusible check` checks every board and reports all violations together. `fusible check sensor` checks one.
- `fusible build sensor-fw` checks the `sensor` board, writes its pin bindings, then builds that firmware.
- `fusible export main --preset fab` writes into `dist/main/fab/`. With several boards, `export` and `studio` need a name.
- A firmware or generated body names its `board:` when the project has several. An authored body may name one too: it is then modeled in that board's coordinate frame. `around:` still works as an alias.
- The old single-board form (`hardware:`, `mcu: { chip }`, one `firmware:`, `mechanical:`) still loads.

### Links between boards

A link says that a connector of one board is cabled to a connector of another.
Each wire joins a pin `a` of `from` to a pin `b` of `to`. A pin is a number
(`1`) or a name (`TX`).

- No `net`: both ends must carry the same net name.
- `net: GND`: both ends must carry `GND`.
- `nets: [TX, RX]`: the first end must carry `TX`, the second `RX`. Use it for crossed wires.

The `harness_mismatch` rule blows the fuse when a connector or pin does not
exist, a pin is wired to nothing, or the nets do not match. It runs after the
boards pass, in `fusible check` and `fusible build`, when every board is checked.

## Commands

### `check`: the electrical linter

```sh
fusible check                       # every board of fusible.yml, then every link
fusible check sensor                # one board
fusible check hardware/index.tsx    # compiles TSX on the fly
fusible check --mcu stm32g431       # also validates pin muxing against the silicon
fusible --input circuit.json        # Circuit JSON directly
```

Compiling TSX stops after 120 seconds. Set `FUSIBLE_BUILD_TIMEOUT_SECS` to change that.

Fusible caches compiled circuits and exports in `.fusible/cache`. Unchanged sources skip `tsci` and print a `⚡` line. Set `FUSIBLE_NO_CACHE=1` to bypass it.

```
the fuse blew: 1 electrical violation(s)
[unconnected_net] net "I2C_SDA" has only 1 connection(s) (minimum 2): it is floating
help: every net must connect at least two pads/ports: complete the routing in tscircuit
```

The rules are listed in [Rules](#rules). A rule that cannot prove a net is
connected reports it. Silence is never the default.

`fusible check --json` prints one JSON document with the verdict and every
diagnostic (rule, message, help, severity, and when known the standard, source
URL, element ids and a fix). The exit codes stay the same.

### `studio`: the studio

```sh
fusible studio               # every board, on port 3020
fusible studio sensor        # opened on one board
fusible studio --port 3030
fusible studio --desktop     # for the desktop shell
```

`dev`, `view`, and `webview` still work as aliases.

- **Project tree.** Lists boards, firmware, and bodies, each with its fuse state. The URL keeps the selection (`?item=board:sensor`).
- **Views.** PCB layout, schematic, 3D board, parts, nets, and checks follow the selected board. Layers toggle on and off.
- **`RUNS`.** Starts `check`, `build`, and `body` from the browser with a live log. It runs the same `fusible` binary, so the result matches the terminal. One run at a time.
- **`PINS`.** Select a firmware to see its pin map: each GPIO by peripheral, signal, connected part, and Rust name. Pins with a `pin_mux` conflict show the reason. `BUILD` runs the firmware build.
- **`BODIES`.** Shows the generated base and lid with lid and explode toggles, or the selected authored body with the outline of its `board:` drawn beside it. GLB loads as is. STEP and STL convert with `occt-wasm`, cached by path, time, and size (64 MiB limit). Authored GLB must be in millimetres, Z-up.
- **`PARTS & COSTS`.** Edits the same `bom.yml` as `fusible bom`.
- **`SYNC`.** Appears when the project is inside a git work tree. `SAVE` stores your changes with a message, gets the others' work, puts yours on top, and sends it. `GET UPDATES` only gets theirs. When two people changed the same file, a card names the file and the author, and `KEEP MINE` or `KEEP THEIRS` decides. `node_modules`, `dist`, `.fusible`, and `*.lck` are never saved. Studio runs the real `git` with no shell and no prompt, so a missing sign-in fails with a message instead of hanging.
- **Narrow windows.** The tree collapses with `[` and becomes a drawer below 900px.

Studio serves only local requests. Run requests accept names from `fusible.yml`, a JSON body, and a local `Origin`.
`fusible studio --share` also listens on your local network for paired phones. See [Phone pairing](#phone-pairing).

### `route`: the autorouter

```sh
fusible route                              # writes dist/circuit.routed.json
fusible route --dry-run                    # checks that every net can route
fusible route --clearance 0.2 --layers 2
```

A multi-layer A* router on R-trees. Each routed trace becomes an obstacle for
the next, so routed nets cannot short. If a net cannot route, the fuse blows and
the command lists the unrouted nets.

### KiCad: `diff`, `sync`, and the studio watch

```sh
fusible diff board.kicad_pcb
fusible sync board.kicad_pcb --write
fusible sync board.kicad_pcb --dry-run
```

While the studio runs, it watches the board's KiCad file. KiCad owns placement.
The TSX owns parts and nets.

- Name the file per board with `kicad:` in `fusible.yml`. Without it, the studio uses the single `.kicad_pcb` beside the entry file or in a `kicad/` folder.
- On save in KiCad, the studio rewrites plain-number `pcbX`, `pcbY`, and `pcbRotation` props in your TSX, then rebuilds. An expression such as `pcbX={base + 2}`, a part without those props, or a flip to the other side is left alone and listed as a difference.
- Before it writes a TSX file, the studio copies it to `.fusible/backup/` and keeps the last 10 per file.
- Added, removed, or renamed parts and changed nets are never written. A `KICAD: n DIFFERENCES` chip lists them. `KICAD: ERROR` means a file could not be read.

### `build`

```sh
fusible build           # every board, then every firmware
fusible build main-fw   # one firmware and its board
```

Checks the boards and links, writes `pins.rs`, and builds the firmware only if
the fuse is intact.

### `export`: fuse-gated deliverables

```sh
fusible export --preset fab    # gerbers.zip + circuit.json into dist/fab/
fusible export --preset cad    # kicad.zip + board.step into dist/cad/
fusible export --preset docs   # schematic.svg, pcb.svg, schematic.pdf into dist/docs/
fusible export --preset all
fusible export -f gerbers -o dist/gerbers.zip
fusible export --preset fab --skip-lint    # drafts only
```

The rules run first. If one fails, no file is written. The PDF needs a local
Chromium; without one, Fusible warns and writes the SVG only. Gerber, drill,
and SVG output are pure Rust.

### `body`: physical bodies

A body is **authored** (`source:`, a file from any CAD tool, with an optional
`build:` command for code-CAD) or **generated** (`generate: enclosure`, a base
and lid around the board outline). Fusible never overwrites an authored file. A
generated body is fuse-gated and regenerated on every run.

```sh
fusible body                       # every body
fusible body case                  # one body
fusible body sensor                # the generated bodies around one board
fusible body -f all                # step, stl, glb, brep into dist/bodies/
fusible body --ventilation grid    # grid | slots
fusible body --wall-thickness 2.5 --height 25 --no-cutouts --no-mounting
```

A board without a body gets a default enclosure. `mechanical`, `case`, and
`housing` are aliases of `body`. The enclosure comes from `pcb_board`: each
`pcb_hole` gets a screw boss, and components that reach the board edge get a
wall notch. Flags override `fusible.yml`, which overrides defaults. It needs
`bun`, and the first run installs `occt-wasm` into `~/.cache/fusible/`.

**Start from the generated body, or design your own:**

```sh
fusible body eject lid                    # writes bodies/lid.step and switches the manifest to source:
fusible body eject lid -o parts/lid.stl   # the extension picks the format
fusible body kit sensor                   # reference kit in bodies/kit/sensor/
```

- `eject` is fuse-gated. It writes the model to a staging file, moves it into place only on success, never replaces an existing file, and edits `fusible.yml` in one step, keeping your comments. If the entry has a shape it cannot edit safely, it leaves the manifest alone and exits 2 with the lines to change by hand.
- `kit` writes `kit.json` for people who model from scratch: the board `outline` and `thickness`, every `hole`, every component footprint (with `z` and `height` when the part has a 3D model), a `keep_out` box per modeled part (`z0` to `z1`), and the wall `openings` where a component reaches an edge. The `notch` block holds the generator's constants. Units are mm in the PCB frame. `board.step` is added for TSX entries when a runner is installed.
- `eject` and `kit` are reserved. A body with either name is rejected.

### `assembly`: boards and bodies in one model

```yaml
assembly:
  out_dir: dist          # default: dist
  format: glb            # glb, step, or all; default: glb
  parts:
    main:   { at: [0, 0, 0] }
    sensor: { at: [0, 0, 12], rotate: [0, 0, 90] }
    enclosure: {}        # a body follows the board it is modeled on
```

```sh
fusible assembly             # writes dist/assembly.glb
fusible asm -f all -o out    # assembly.glb and assembly.step in out/
```

Units are mm. `rotate` is in degrees around the fixed X, then Y, then Z axes,
and applies before `at`. A body with a board (its `board:`, or the only board
for a generated body) sits on that board, and its own `at` and `rotate` are
added on top. That board must be a part too. A body without a board needs an
`at`. A board left out of `parts` is not drawn.

The command is fuse-gated: every board it draws, and every board a generated
body is fitted to, passes `check` first (`--skip-lint` skips it). A board needs
its `.tsx` entry and a tscircuit runner, which exports its STEP model. A
generated body is rebuilt for the assembly; an authored body is read from its
`source:` (STEP or STL) after its `build:` runs. The models of each part land in
`dist/assembly/`. Like `body`, it needs `bun` and `occt-wasm`.

Three rules compare the boards with the bodies and blow the fuse (exit 1, no
`assembly.*` written):

- `body_interference`: a body overlaps the board slab or a component's volume,
  or two bodies overlap.
- `hole_misaligned`: a body has material near a board hole but no boss around
  it (fewer than 2 of 4 ring quadrants), or has any material inside the hole's
  bore (a pilot hole up to 0.5 mm narrower in radius is fine). A body fitted to a board (generated, or authored with `board:`) must also have a boss for every non-plated hole of that board: a hole with no body material near it blows the fuse. Plated holes (vias, pads) need no boss.
- `connector_blocked`: a body stands in front of the opening a connector at the
  board edge needs.

Parts that only touch do not fire. Two bodies that overlap by more than 0.01 mm³ blow the fuse, so give a lip a clearance. Each run writes `dist/assembly/check.json`
(`fuse`, and per violation the rule, parts, overlap volume in mm³, and position)
and `dist/assembly/preview.glb`, which holds the bodies, the board slabs, and one
`collision:<n>` node per overlap, even when the fuse blows. `fusible check` runs
the rules too, when it checks every board. Without `assembly:` they are skipped.
A declared `assembly:` without `bun`, or a body that is not a closed solid, is a
tool error (exit 2), never a pass.

### `bom`: parts and costs

```sh
fusible bom                         # every board
fusible bom main -f csv -o parts.csv
fusible bom -f json
```

Lists the placed components grouped by kind, value, and part number, and adds
each body as a 3D print. `bom.yml` holds prices, picks, and manual parts:

```yaml
currency: EUR
print:
  price_per_kg: 25
parts:
  resistor-10kohm: {unit_price: 0.02, supplier: LCSC}
  m3-screw: {description: M3x8 screw, quantity: 8, unit_price: 0.05}
prints:
  case: {grams: 42}
```

A key that matches a detected part overrides it. Any other key is a manual
part. Print cost is `grams / 1000 * price_per_kg`. A part without a price gets
an estimate, marked `~`. `bom` never blows the fuse.

### Packages and toolchain

```sh
fusible search esp32-s3
fusible add @tsci/seveibar.smd-resistor
fusible install
fusible doctor --json      # {"bun","node","tsci"}, exit 0
```

`FUSIBLE_BUN` names the bun binary for every command. Without a `tsci`, Fusible
runs `bun x --bun --package tscircuit tsci`.

## Rules

Every rule runs in the fuse. Errors blow it. Warnings print. Rules that need your
numbers stay off until you declare them in `fusible.yml`.

| Rule | Checks | Source |
|---|---|---|
| `trace_current` | Temperature rise of the narrowest trace of a net that declares `current`. | IPC-2152 (Brooks and Adam fit, as in KiCad): https://docs.kicad.org/doxygen/tracks__width__versus__current__formula_8h.html |
| `voltage_clearance` | Copper spacing between nets for their voltage difference, class B1, B2 or B4. | IPC-2221B Table 6-1 |
| `fab_profile` | Trace width and spacing, drill, via pad, annular ring, via to via, board size for `jlcpcb-2l`, `jlcpcb-4l` or your own limits. | https://jlcpcb.com/capabilities/pcb-capabilities |
| `trace_clearance` | Copper clearance between different nets, 0.15 mm unless a fab profile is set. | |
| `decoupling_capacitor` | Every IC supply pin has a bypass capacitor, within 3 mm (warning). | https://www.protoexpress.com/blog/decoupling-capacitor-placement-guidelines-pcb-design/ |
| `autoroute` | tscircuit placement or routing errors, and nets with no routed trace. | |
| `unconnected_net`, `ghost_net`, `shorted_nets`, `malformed_element`, `dangling_reference` | Netlist integrity. | |
| `pin_mux` | Pin muxing against the silicon of the target MCU. | |
| `qfn_footprint_span` | A qfn footprint that gives only a copper span (warning). | |
| `E_SCHEMATIC_SCALE_EXCESSIVE` | A schematic too large for one sheet. | |
| `harness_mismatch` | Cabled connectors that do not match. | |
| `body_interference`, `hole_misaligned`, `connector_blocked` | Boards against bodies in an assembly. | |
| `mech.*` | Wall thickness, bosses, port and component clearance, vents, standoff height and draft, by process. | see [Mechanical parts](#mechanical-parts) |

Declare the numbers a rule needs:

```yaml
nets:
  VBUS: { voltage: 5, current: 2, max_rise: 10 }   # trace_current, voltage_clearance
  MAINS_L: { voltage: 230 }
fab: jlcpcb-2l                # or jlcpcb-4l, or { min_trace: 0.15, min_space: 0.15, ... }
clearance_class: B2           # B1 internal, B2 external uncoated (default), B4 coated
copper: { outer_oz: 1, inner_oz: 0.5 }
```

These are design pre-checks, not certifications. Fusible does not check
creepage under IEC 62368-1 yet. A product with a safety rating still needs a lab.
Fab profiles carry a source URL and the date they were checked.

## Mechanical parts

Beyond the generated enclosure, a body can be a TypeScript module on the same
OpenCascade kernel, modeled against the real board:

```sh
fusible body new bracket --material petg --process fdm
fusible body bracket                 # writes STEP, STL and GLB into dist/bodies/
```

```yaml
bodies:
  bracket:
    source: mech/bracket.ts
    material: petg        # pla, petg, abs, asa, pa12, resin, pc, al6061
    process: fdm          # fdm, sla, mjf, injection or cnc
```

The part receives the board outline, holes, component heights and openings, and
helpers for shells, screw bosses, standoffs, vents and cut-outs. Builds are
reproducible: STEP files carry a fixed timestamp. Materials feed `fusible bom`
with mass and cost. The `mech.*` rules read the features the helpers record, and
the process decides the limits (for example a 2 mm wall on FDM, 1.5 mm on SLA).
Sources for every limit are in [docs/v1/E-mechanical.md](docs/v1/E-mechanical.md).

## Agents

`fusible mcp` is a Model Context Protocol server over stdio. It exposes checks, rule
explanations, components, nets, the BOM, firmware pins, fuse-gated edits to boards
and parts, builds, exports and SVG renders. Every edit returns the diff and the
new fuse verdict, and `export` refuses to run while the fuse is blown. Edits are
file changes in your git repository. Nothing is committed for you.

```sh
fusible mcp --print-config bulle     # or claude, or codex
```

```yaml
sources:
  mcp:
    fusible:
      command: fusible
      args: [mcp]
```

That snippet goes in `~/.config/bulle/config.yml`. See [docs/agents.md](docs/agents.md)
for the tool list and the write-safety rules. Research on PCB agents finds that
agents with rule-checker feedback beat one-shot generation:
https://arxiv.org/html/2607.05915 and https://arxiv.org/html/2602.00510v1.

## Phone pairing

```sh
fusible studio --share
```

`--share` also listens on your local network, which must be a private address.
Press **Pair phone** in the studio to show a QR code. The Android app scans it and
gets a device token. A paired phone can read the project summary, board data, BOM
and checks, and nothing else: every other route, and every method except GET,
is refused by the server. Codes last five minutes and work once. The desktop app
has a switch for this in its settings.

## Release

```sh
fusible release                    # dist/release/ with fab files, BOM, pick and place, RELEASE.txt
fusible release --allow-warnings
```

`release` checks every board, requires routed traces, exports the fab preset, and
checks that copper pours made it into the gerbers. It writes nothing when any step
fails.

## Facile suite

```sh
fusible check --opus                       # a task in Opus for each new blown-fuse error
fusible bom --opus                         # a task for each unpriced part
fusible share nuage --space Kono --link    # export the fab files into a Nuage space
```

These call the `opus` and `nuage` command line tools you already signed in to.
Without them, the fuse verdict is unchanged.

## Exit codes

| Code | Meaning |
|---|---|
| `0` | Fuse intact. The command succeeded. |
| `1` | Fuse blown. A rule failed and generation is blocked. |
| `2` | Tool error. Unreadable input, missing runner, or bad arguments. |

CI can tell a bad design (1) from a broken setup (2).

## CI

The repository is also a GitHub Action. It installs a release binary, sets up
bun, runs `fusible check`, and fails the job when the fuse blows:

```yaml
- uses: actions/checkout@v7
- id: fuse
  uses: FacileStudio/Fusible@v1
  with:
    target: ""        # optional board name or file; empty checks every board
    version: latest   # or a version such as 1.0.0
```

Other inputs: `working-directory` (default `.`) and `install-deps` (default
`true`, runs `bun install` where a `package.json` sits). The `fuse` output is
`intact`, `blown`, or `error`, from exit codes 0, 1, and 2.

## Layout of the source

```
src/
├── cli.rs, dispatch.rs, main.rs   # commands and exit codes
├── commands/                      # check, build, export, body, bom, release, mcp, share, links, init, doctor
├── config/                        # fusible.yml: boards, firmware, bodies, links
├── parser/, netlist/, linter/     # pure Rust: Circuit JSON, net graph, rules
├── silicon/, codegen/             # pin muxing databases and pins.rs
├── gerber/, svg/, router/, kicad/ # fabrication, rendering, routing, KiCad sync
├── dev/                           # studio server (tiny_http, SSE, git, KiCad watch, pairing)
├── tscircuit/                     # runner discovery and process control
└── assets/                        # embedded studio
studio/                            # Svelte studio, built into src/assets/studio.bundle.js
desktop/                           # Tauri shell and its Svelte UI
mobile/                            # Capacitor Android viewer
site/                              # fusible.facile.studio
scripts/                           # screenshots
```

The parser, netlist, and rules have no C dependencies. tscircuit, bun, and
`occt-wasm` are used only for TSX, STEP, and bodies.

## Development

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --locked
filet check .
```

Rebuild the studio bundle with `cd studio && bun run build`. See
[ROADMAP.md](ROADMAP.md) for what is next and [CHANGELOG.md](CHANGELOG.md) for
what changed.

## License

[MIT](LICENSE)
