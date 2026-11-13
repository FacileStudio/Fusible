# Fusible roadmap

Fusible checks tscircuit projects against electrical, physical and mechanical rules, and blocks exports, firmware bindings and flashing while a rule fails. This page describes the 2.0 project, the commands it adds, and what comes after.

## The 2.0 project

`fusible new <name> --template esp32-s3|stm32g4|rp2040|blank` writes this layout:

```text
my-project/
├── fusible.yml                    # boards, firmware, bodies, rules, waivers
├── hardware/
│   ├── index.tsx                  # the board, with pinAttributes on the MCU and regulator
│   ├── package.json               # tscircuit, pinned
│   └── dist/index/circuit.json    # pre-built, so check passes before any install
└── firmware/                      # embedded Rust that blinks the board LED
    ├── Cargo.toml                 # embassy-stm32, embassy-rp or esp-hal, pinned
    ├── .cargo/config.toml         # target and probe-rs or espflash runner
    ├── rust-toolchain.toml
    └── src/{main.rs,pins.rs,board.rs}  # generated bindings; main.rs calls board::take!(p)
```

The manifest:

```yaml
name: sensor-node

boards:
  main:
    entry: hardware/index.tsx
    out_dir: dist
    mcu: esp32-s3

firmware:
  app:
    board: main
    path: firmware
    framework: esp-hal          # embassy | cortex-m | esp-hal | esp-idf
    pins_rs: firmware/src/pins.rs
    outputs: [rust]
    build_cmd: cargo build --release
    flash_cmd: cargo run --release

bodies:
  enclosure:
    generate: enclosure
    board: main

rules:
  decoupling_capacitor: warning # off | info | warning | error

waivers:
  - rule: decoupling_capacitor
    match: U1.VDD               # element id or REFDES.PIN
    reason: bulk capacitor shared with U2
    until: 2027-03-01           # optional
```

A 1.x manifest (`hardware:`, `mcu: { chip }`, one `firmware:`, `mechanical:`) still loads. `fusible migrate` rewrites it; see [docs/migrate-2.0.md](docs/migrate-2.0.md).

## Commands added in 2.0

| Command | What it does |
|---|---|
| `fusible new <name> --template <t>` | A project from a starter design. |
| `fusible rules [--json]` | Every rule with its category and level. |
| `fusible explain <rule> [--json]` | One rule: summary, standard, source, level, whether it can be waived. |
| `fusible migrate [--check]` | Rewrites a 1.x `fusible.yml`, comments kept. |
| `fusible check --write-bindings` | Writes `pins.rs` once the fuse is intact. A plain `check` writes nothing. |
| `fusible flash [FIRMWARE]` | Fuse-gated, then runs the firmware's `flash_cmd`, by default `probe-rs run --chip <chip>` or `espflash flash`. |
| `fusible bringup [BOARD]` | Writes `dist/bringup.md`: rails in power order with the voltage and test point to probe, strap pin states, debug connector pinout. |

MCP gains `apply_fix` (applies a `set_prop` fix) and `waive` (adds a reasoned waiver). `check` returns the version 2 report.

## 2.0 milestones

- [x] **Foundations.** Typed Circuit JSON model, pin attributes and part data, one check pipeline for the CLI, `--json`, the studio and MCP, a rule registry with stable ids and categories, located diagnostics with machine fixes, `info` severity, rule levels and waivers, `fusible migrate`.
- [x] **Electrical rules.** Pin roles from `pinAttributes`, pin conflicts, undriven power inputs, unconnected and no-connect pins, missing pull-ups, voltage domains, rail budget, capacitor derating, LED series resistors, bring-up warnings.
- [x] **Fabrication and assembly.** Annular ring, drill to copper, hole spacing, via aspect ratio, edge clearance, silkscreen, mask slivers, acid traps, rotation-aware courtyards, fiducials, BOM completeness, IPC-D-356 netlist and centroid files.
- [x] **Firmware bridge.** Generated STM32 pin tables, one bindings model rendered as Rust, a C header and a Zephyr overlay, `fusible flash`, `fusible bringup`.
- [x] **Agent and human experience.** `fusible new` with four templates, `fusible rules` and `fusible explain`, generated `docs/rules/`, MCP `apply_fix` and `waive`, studio Checks grouped by category with zoom, waivers and Apply fix.
- [ ] **Integration.** 2.0.0 shipped. Left: FieldCam acceptance, where the board runs `check`, `export`, `bringup`, `build` and `bom` with the real SIM7600G-H land pattern. The land pattern is drawn and checked against SIMCom's hardware design V1.03, but with it the board no longer routes: tscircuit's `beta_pipeline9` autorouter aborts on the hand-routed `VBAT_MODEM` feed ("cannot reconnect preloaded trace").

The workspace split (`fusible-core` for `wasm32`) is dropped: `fusible` stays the only published crate.

## After 2.0

- [x] Group the studio server modules in `src/dev/` by feature: `http/`, `live/` and `api/` beside the feature folders.
- [x] Render an exported Gerber set in a real viewer (`gerbv`) in CI. CI caches the gerbv packages, so a cold apt mirror only matters on a cache miss.

## Done before 2.0

1.x shipped the parser and fail-closed linter, the exit code contract, the `fusible.yml` manifest with named boards, firmware and bodies, the tscircuit runner and build cache, pure Rust Gerber, Excellon and SVG, the silicon pin-mux engine for STM32G431, RP2040 and ESP32-S3, typed `pins.rs`, the studio with KiCad watch and git sync, the enclosure generator and code-first bodies, assemblies with cross-world rules, the BOM, the release gate, the GitHub Action, the MCP server, the desktop app with signed updates and the Android viewer. [CHANGELOG.md](CHANGELOG.md) has the details.

## Not planned

These stay out until a real project needs them:

- A Fusible parts library. Templates and `pinAttributes` carry part intent.
- A constraint solver or automatic part picking.
- SPICE, signal integrity, impedance and length matching. They need a stack-up model first.
- Crystal load capacitance, power sequencing, I2C address conflicts. tscircuit's `<drccheck>` covers project-specific checks today.
- A plugin system for third-party rules.
- Panelisation, ODB++, IPC-2581, fab drawings as PDF.
- Arduino and PlatformIO generators. `pins.h` serves both.
