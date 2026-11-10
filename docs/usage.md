# Command reference

Everything `fusible` accepts, command by command. The [README](../README.md) holds the
install, the project file and the rules; this file holds the flags and the behaviour of each
command.

## `check`: the electrical linter

```sh
fusible check                       # every board of fusible.yml, then every link
fusible check sensor                # one board
fusible check hardware/index.tsx    # compiles TSX on the fly
fusible check --mcu stm32g431       # also validates pin muxing against the silicon
fusible check --write-bindings      # also writes pins.rs once the fuse is intact
fusible --input circuit.json        # Circuit JSON directly
```

`check` writes no file unless `--write-bindings` (or `--pins-rs PATH`) asks for the pin
bindings; `fusible build` always writes them.

```
the fuse blew: 1 electrical violation(s)
[unconnected_net] net "I2C_SDA" has only 1 connection(s) (minimum 2): it is floating
help: every net must connect at least two pads/ports: complete the routing in tscircuit
```

Compiling TSX stops after 120 seconds (`FUSIBLE_BUILD_TIMEOUT_SECS`). Compiled circuits and
exports are cached in `.fusible/cache`; unchanged sources skip `tsci` and print a `⚡` line
(`FUSIBLE_NO_CACHE=1` bypasses the cache). `--json` prints one report, version 2, and nothing
else on stdout: `fuse`, `boards[]` with their diagnostics (rule, message, help, severity, and
when known the standard, source URL, element ids and a fix), `links[]`, `assembly[]`,
`waived[]`, `unused_waivers[]` and `expired_waivers[]`. Exit codes stay the same.

### Rule levels and waivers

```yaml
rules:
  decoupling_capacitor: warning   # off | info | warning | error
waivers:
  - rule: decoupling_capacitor
    match: "U3.VDD"               # element id, part name, or REFDES.PIN
    reason: "bulk cap C12 shared, checked on rev A"
    until: 2027-03-01             # optional, last day it applies
```

A waiver applies when its rule matches and `match` names an element id of the diagnostic,
the name of a listed part, or `REFDES.PIN` (`U3.VDD` or `U3.4`) of a listed port. For rules
that list the part and not the port, `REFDES.PIN` also matches when the message names that
pin. A waived diagnostic leaves the verdict and shows up under "waived". An unknown rule id
is a tool error (exit 2). A waiver that matches nothing prints an `unused_waiver` warning.
An expired waiver no longer applies. Netlist rules can be neither lowered nor waived.

A rule that cannot prove a net is connected reports it. Silence is never the default.

## `studio`: the studio

```sh
fusible studio               # every board, on port 3020
fusible studio sensor        # opened on one board
fusible studio --port 3030
fusible studio --desktop     # for the desktop shell
```

`dev`, `view`, and `webview` still work as aliases.

- **Project tree.** Boards, firmware and bodies, each with its fuse state. The URL keeps the selection (`?item=board:sensor`).
- **Views.** PCB layout, schematic, 3D board, parts, nets and checks follow the selected board. Layers toggle on and off.
- **`RUNS`.** Starts `check`, `build` and `body` from the browser with a live log. Same `fusible` binary, so the result matches the terminal. One run at a time.
- **`PINS`.** A firmware's pin map: each GPIO by peripheral, signal, connected part and Rust name, with the reason for every `pin_mux` conflict. `BUILD` runs the firmware build.
- **`BODIES`.** Generated base and lid with lid and explode toggles, or the selected authored body beside the outline of its `board:`. GLB loads as is (millimetres, Z-up); STEP and STL convert with `occt-wasm`, cached by path, time and size (64 MiB limit).
- **`PARTS & COSTS`.** Edits the same `bom.yml` as `fusible bom`.
- **`SYNC`.** Appears when the project is inside a git work tree. `SAVE` stores your changes with a message, gets the others' work, puts yours on top, and sends it; `GET UPDATES` only gets theirs. When two people changed the same file, a card names it and `KEEP MINE` or `KEEP THEIRS` decides. `node_modules`, `dist`, `.fusible` and `*.lck` are never saved. Studio runs the real `git` with no shell and no prompt, so a missing sign-in fails with a message instead of hanging.
- **Narrow windows.** The tree collapses with `[` and becomes a drawer below 900px.

Studio serves only local requests. Run requests accept names from `fusible.yml`, a JSON body,
and a local `Origin`. `fusible studio --share` also listens on your local network for paired
phones. See [Phone pairing](#phone-pairing).

## `route`: the autorouter

```sh
fusible route                              # writes dist/circuit.routed.json
fusible route --dry-run                    # checks that every net can route
fusible route --clearance 0.2 --layers 2
```

A multi-layer A* router on R-trees. Each routed trace becomes an obstacle for the next, so
routed nets cannot short. If a net cannot route, the fuse blows and the command lists the
unrouted nets.

## KiCad: `diff`, `sync`, and the studio watch

```sh
fusible diff board.kicad_pcb
fusible sync board.kicad_pcb --write
fusible sync board.kicad_pcb --dry-run
```

KiCad owns placement; the TSX owns parts and nets. Name the file per board with `kicad:` in
`fusible.yml` — without it the studio uses the single `.kicad_pcb` beside the entry file or in
a `kicad/` folder.

On save in KiCad, the studio rewrites plain-number `pcbX`, `pcbY` and `pcbRotation` props in
your TSX, then rebuilds. An expression such as `pcbX={base + 2}`, a part without those props,
or a flip to the other side is left alone and listed as a difference. Before it writes a TSX
file the studio copies it to `.fusible/backup/` and keeps the last 10 per file. Added,
removed or renamed parts and changed nets are never written: a `KICAD: n DIFFERENCES` chip
lists them, and `KICAD: ERROR` means a file could not be read.

## `build`

```sh
fusible build           # every board, then every firmware
fusible build main-fw   # one firmware and its board
```

Checks the boards and links, then, only if the fuse is intact, writes `pins.rs` and builds
the firmware.

### Zephyr overlay (experimental)

`outputs: [zephyr]` writes `app.overlay` in the firmware directory. Fusible checks it against
Zephyr's devicetree bindings during development, but no Zephyr build checks it. Review it
before you flash.

The overlay holds:

- Each I2C, SPI and UART bus with its pinctrl (`hal_stm32` labels on STM32, a
  `<bus>_fusible` state on RP2040, RP2350A, RP2350B, ESP32-S3 and ESP32-C3) and
  `status = "okay"`. SPI buses get `cs-gpios`.
- One child node per chip on a bus. `reg` is the I2C address or the chip-select index.
- One ADC channel node per analog input, its `io-channels` entry under `zephyr,user`, and a
  `voltage-divider` node when two resistors divide the input.
- `gpio-leds` and `gpio-keys` nodes for LEDs and buttons whose polarity the netlist shows.
  Other free pins become `<signal>-gpios` under `zephyr,user`.

Fusible never guesses a `compatible` or an address. It takes them from the manifest, else
from a small table of known parts: BME280, SHT30/31/35, MPU-6050, BH1750, VL53L0X, PCF8563,
u-blox M8 and M10. For a part with an address strap pin (BME280 `SDO`, MPU-6050 `AD0`),
the netlist must tie that pin to ground or a supply. Any other chip gets a disabled node
marked `FUSIBLE TODO`, and `fusible build` prints a `warning:` line naming the facts to add:

```yaml
firmware:
  app:
    framework: zephyr
    outputs: [zephyr]
    zephyr:
      U3: { compatible: "jedec,spi-nor", spi-max-frequency: 8000000 }
      U5: { compatible: "ti,tmp11x", reg: 0x48 }
```

Each device node is labelled with its designator (`u3:`), so a second overlay of your own
can add properties the binding needs, such as `size` and `jedec-id` for `jedec,spi-nor`.
ADC channels use unity gain, the internal reference and the default acquisition time; check
them against the input range. ESP32 ADC inputs get a TODO: the ESP32 tables carry no ADC
channel numbers yet.

## `export`: fuse-gated deliverables

```sh
fusible export --preset fab    # gerbers.zip + circuit.json into dist/fab/
fusible export --preset cad    # kicad.zip + board.step into dist/cad/
fusible export --preset docs   # schematic.svg, pcb.svg, schematic.pdf into dist/docs/
fusible export --preset all
fusible export -f gerbers -o dist/gerbers.zip
fusible export --preset fab --skip-lint    # drafts only
```

The rules run first; if one fails, no file is written. The PDF needs a local Chromium —
without one, Fusible warns and writes the SVG only. Gerber, drill and SVG output are pure
Rust.

## `body`: physical bodies

A body is **authored** (`source:`, a file from any CAD tool, with an optional `build:` command
for code-CAD) or **generated** (`generate: enclosure`, a base and lid around the board
outline). Fusible never overwrites an authored file; a generated body is fuse-gated and
regenerated on every run.

```sh
fusible body                       # every body
fusible body case                  # one body
fusible body sensor                # the generated bodies around one board
fusible body -f all                # step, stl, glb, brep into dist/bodies/
fusible body --ventilation grid    # grid | slots
fusible body --wall-thickness 2.5 --height 25 --no-cutouts --no-mounting
```

A board without a body gets a default enclosure. `mechanical`, `case` and `housing` are
aliases of `body`. The enclosure comes from `pcb_board`: each `pcb_hole` gets a screw boss, and
components that reach the board edge get a wall notch. Flags override `fusible.yml`, which
overrides defaults. It needs `bun`; the first run installs `occt-wasm` into
`~/.cache/fusible/`.

Start from the generated body, or design your own:

```sh
fusible body eject lid                    # writes bodies/lid.step, switches the manifest to source:
fusible body eject lid -o parts/lid.stl   # the extension picks the format
fusible body kit sensor                   # reference kit in bodies/kit/sensor/
```

`eject` is fuse-gated: it writes to a staging file, moves it into place only on success, never
replaces an existing file, and edits `fusible.yml` in one step, keeping your comments. If the
entry has a shape it cannot edit safely, it leaves the manifest alone and exits 2 with the
lines to change by hand. `kit` writes `kit.json` for people who model from scratch: the board
`outline` and `thickness`, every `hole`, every component footprint (with `z` and `height` when
the part has a 3D model), a `keep_out` box per modeled part, and the wall `openings` where a
component reaches an edge — mm, in the PCB frame. `board.step` is added for TSX entries when a
runner is installed. `eject` and `kit` are reserved: a body with either name is rejected.

## `assembly`: boards and bodies in one model

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

Units are mm. `rotate` is in degrees around the fixed X, then Y, then Z axes, and applies
before `at`. A body with a board sits on that board (its own `at` and `rotate` add on top, and
that board must be a part too); a body without a board needs an `at`. A board left out of
`parts` is not drawn.

The command is fuse-gated: every board it draws, and every board a generated body is fitted
to, passes `check` first (`--skip-lint` skips it). A board needs its `.tsx` entry and a
tscircuit runner; a generated body is rebuilt, an authored body read from its `source:` after
its `build:` runs. Models land in `dist/assembly/`. Like `body`, it needs `bun` and
`occt-wasm`.

Three rules compare the boards with the bodies and blow the fuse (exit 1, no `assembly.*`
written):

- `body_interference`: a body overlaps the board slab or a component's volume, or two bodies overlap.
- `hole_misaligned`: a body has material near a board hole but no boss around it (fewer than 2 of 4 ring quadrants), or material inside the bore (a pilot hole up to 0.5 mm narrower in radius is fine). A body fitted to a board must also have a boss for every non-plated hole of that board; plated holes need none.
- `connector_blocked`: a body stands in front of the opening a connector at the board edge needs.

Parts that only touch do not fire. Two bodies overlapping by more than 0.01 mm³ blow the fuse,
so give a lip a clearance. Each run writes `dist/assembly/check.json` (the fuse, and per
violation the rule, parts, overlap volume and position) and `dist/assembly/preview.glb`, which
holds the bodies, the board slabs and one `collision:<n>` node per overlap, even when the fuse
blows. `fusible check` runs these rules too, when it checks every board; without `assembly:`
they are skipped. A declared `assembly:` without `bun`, or a body that is not a closed solid,
is a tool error (exit 2), never a pass.

## `bom`: parts and costs

```sh
fusible bom                         # every board
fusible bom main -f csv -o parts.csv
fusible bom -f json
```

Lists the placed components grouped by kind, value and part number, and adds each body as a 3D
print. `bom.yml` holds prices, picks and manual parts:

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

A key that matches a detected part overrides it; any other key is a manual part. Print cost is
`grams / 1000 * price_per_kg`. A part without a price gets an estimate, marked `~`. `bom`
never blows the fuse.

## Packages and toolchain

```sh
fusible search esp32-s3
fusible add @tsci/seveibar.smd-resistor
fusible install
fusible doctor --json      # {"bun","node","tsci"}, exit 0
```

`FUSIBLE_BUN` names the bun binary for every command. Without a `tsci`, Fusible runs
`bun x --bun --package tscircuit tsci`.

## Mechanical parts

Beyond the generated enclosure, a body can be a TypeScript module on the same OpenCascade
kernel, modeled against the real board:

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

The part receives the board outline, holes, component heights and openings, plus helpers for
shells, screw bosses, standoffs, vents and cut-outs. Builds are reproducible: STEP files carry
a fixed timestamp. Materials feed `fusible bom` with mass and cost, and the `mech.*` rules
read the features the helpers record — the process decides the limits (a 2 mm wall on FDM, 1.5
mm on SLA). Sources for every limit are in [docs/v1/E-mechanical.md](v1/E-mechanical.md).

## Agents

`fusible mcp` is a Model Context Protocol server over stdio. It exposes checks, rule
explanations, components, nets, the BOM, firmware pins, fuse-gated edits to boards and parts,
builds, exports and SVG renders. Every edit returns the diff and the new fuse verdict, and
`export` refuses to run while the fuse is blown. Edits are file changes in your git
repository; nothing is committed for you.

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

That snippet goes in `~/.config/bulle/config.yml`. See [agents.md](agents.md) for the tool
list and the write-safety rules. Research on PCB agents finds that agents with rule-checker
feedback beat one-shot generation: https://arxiv.org/html/2607.05915 and
https://arxiv.org/html/2602.00510v1.

## Phone pairing

```sh
fusible studio --share
```

`--share` also listens on your local network, which must be a private address. Press **Pair
phone** in the studio to show a QR code; the Android app scans it and gets a device token. A
paired phone can read the project summary, board data, BOM and checks, and nothing else: every
other route, and every method except GET, is refused by the server. Codes last five minutes
and work once. The desktop app has a switch for this in its settings.

## Release

```sh
fusible release                    # dist/release/ with fab files, BOM, pick and place, RELEASE.txt
fusible release --allow-warnings
```

`release` checks every board, requires routed traces, exports the fab preset, and checks that
copper pours made it into the gerbers. It writes nothing when any step fails.

## Facile suite

```sh
fusible check --opus                       # a task in Opus for each new blown-fuse error
fusible bom --opus                         # a task for each unpriced part
fusible share nuage --space Facile --link    # export the fab files into a Nuage space
```

These call the `opus` and `nuage` command line tools you already signed in to. Without them,
the fuse verdict is unchanged.