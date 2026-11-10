# `voltage_clearance`

Copper at different declared voltages keeps the IPC-2221B Table 6-1 spacing.

| Field | Value |
| --- | --- |
| Category | drc |
| Default severity | error |
| Standard | IPC-2221B |
| Source | https://www.atlaspcb.com/tools/conductor-spacing-calculator |

## Change its level

```yaml
rules:
  voltage_clearance: warning

waivers:
  - rule: voltage_clearance
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain voltage_clearance` for the level this project gives it.
