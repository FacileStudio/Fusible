# Migrate to Fusible 2.0

Fusible 2.0 breaks four things: some rule ids, the JSON report, where `pins.rs` gets written, and the old single-board manifest form. Most projects need two commands:

```sh
fusible migrate --check   # exit 1 when fusible.yml needs a rewrite, writes nothing
fusible migrate           # rewrites fusible.yml and prints the diff
```

Then update any script that reads `fusible check --json` or expects `check` to write `pins.rs`.

## Rewrite the manifest

`fusible migrate` finds the nearest `fusible.yml` and rewrites it in place. It works on the text, so every comment and every line outside the old sections stays as it was.

It moves the 1.x single-board sections under named entries:

| 1.x | 2.0 |
|---|---|
| `hardware:` | `boards: main:` |
| `mcu: { chip: X }` | `mcu: X` on that board |
| one unnamed `firmware:` | `firmware: firmware:` |
| `mechanical:` | `bodies: <project name>:` with `generate: enclosure` |

It also renames old rule ids. The rewrite must load to the same configuration as the old file, or nothing is written and the command exits 2. When that happens, edit the file by hand.

## Rule ids

| 1.x | 2.0 |
|---|---|
| `E_SCHEMATIC_SCALE_EXCESSIVE` | `schematic_scale` |
| `autoroute` | `unrouted`, plus one id per tscircuit error kind |
| `bypass_cap` (source file name) | `decoupling_capacitor` (the id did not change) |

`autoroute` used to report every tscircuit `*_error` element under one id. In 2.0:

- `unrouted` blows the fuse when a board has nets but no routed trace, or routed copper does not reach every port of a net (judged per net, whatever the `pcb_trace` ids).
- Each tscircuit error kind has its own id: `footprint_overlap`, `courtyard_overlap`, `placement`, `off_board`, `trace_error`, `trace_missing`, `port_not_connected`, `via_clearance`, `pad_trace_clearance`, `via_trace_clearance`, `pad_pad_clearance`, `autorouting`, `connector_accessibility`.
- Any other `*_error` still blows the fuse, as `upstream_error`.
- tscircuit warnings print as `upstream_warning`. Unnamed traces print as `unnamed_trace`, at info level.

Run `fusible rules` for the full list, and `fusible explain <rule>` for one rule. Each rule has a page in [rules/](rules/README.md).

## Rule levels and waivers

Two manifest keys are new:

```yaml
rules:
  decoupling_capacitor: warning   # off | info | warning | error

waivers:
  - rule: decoupling_capacitor
    match: U3.VDD                 # element id or REFDES.PIN, exact
    reason: bulk capacitor C12 is shared, checked on rev A
    until: 2027-03-01             # optional, last day it applies
```

- `info` never blows the fuse. `warning` prints. `error` blows the fuse. `off` does not run the rule.
- A waiver needs a `reason`. It moves the matching diagnostic to the report's `waived` list. It does not hide it.
- A waiver past its `until` day no longer applies and is listed under `expired_waivers`.
- A waiver that matches nothing is an `unused_waiver` warning.
- An unknown rule id in `rules:` or `waivers:` is exit 2.
- Netlist rules (`unconnected_net`, `malformed_element`, `ghost_net`, `dangling_reference`, `shorted_nets`) stay errors. No waiver applies to them.

An agent can add a waiver through the MCP `waive` tool. It refuses an empty reason.

## `check` no longer writes `pins.rs`

In 1.x, a passing `fusible check` wrote the firmware pin bindings. In 2.0 `check` writes no file.

- `fusible build` writes `pins.rs`, then builds the firmware.
- `fusible check --write-bindings` writes `pins.rs` once the fuse is intact.
- `fusible check --pins-rs PATH` writes it to `PATH` and implies `--write-bindings`.

If CI ran `fusible check` and then compiled firmware, add `--write-bindings` or switch to `fusible build`.

## The JSON report, version 2

`fusible check --json` and the MCP `check` tool print one document:

```json
{
  "version": 2,
  "fuse": "blown",
  "boards": [{ "name": "main", "diagnostics": [] }],
  "links": [],
  "assembly": [],
  "waived": [{ "board": "main", "diagnostic": {}, "reason": "...", "until": null }],
  "unused_waivers": [],
  "expired_waivers": []
}
```

- `fuse` is `intact` or `blown`. A tool error prints `{"fuse": "error", "error": "..."}` and exits 2.
- Every diagnostic carries every field, `null` when unknown: `rule`, `message`, `help`, `severity` (`error`, `warning` or `info`), `standard`, `source_url`, `elements`, `fix`, `location` (`board`, `x`, `y`, `layer`), `source` (`file`, `line`) and `fixes`.
- `fixes` lists machine-applicable changes: `{"set_prop": {"selector", "prop", "value"}}`, `{"add_element": {"tsx"}}` or `{"note": "..."}`. The MCP `apply_fix` tool and the studio's Apply fix button apply `set_prop`.
- `--json` writes nothing else on stdout. Logs go to stderr.

The 1.x document held only `fuse` and `boards[]`. A script that read `boards[].diagnostics` keeps working, but link and assembly diagnostics now sit in `links` and `assembly`, and waived ones in `waived`. Check `version` before reading the rest.

## Start a new project instead

`fusible new <name> --template esp32-s3|stm32g4|rp2040|blank` writes a 2.0 project from scratch: a board with `pinAttributes` on the MCU and the regulator, an embedded firmware crate (Embassy, or `esp-hal` on the ESP32-S3) that blinks the LED through the generated bindings, a generated enclosure and a `fusible.yml`. Each template ships a pre-built Circuit JSON, so `fusible check` passes before any install.
