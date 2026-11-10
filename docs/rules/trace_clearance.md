# `trace_clearance`

Copper of distinct nets on one layer keeps 0.15 mm, or the fab minimum space when a fab is set.

| Field | Value |
| --- | --- |
| Category | drc |
| Default severity | error |

## Change its level

```yaml
rules:
  trace_clearance: warning

waivers:
  - rule: trace_clearance
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain trace_clearance` for the level this project gives it.
