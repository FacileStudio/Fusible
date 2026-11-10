# `via_trace_clearance`

tscircuit found a trace too close to a via.

| Field | Value |
| --- | --- |
| Category | upstream |
| Default severity | error |
| Standard | tscircuit |

## Change its level

```yaml
rules:
  via_trace_clearance: warning

waivers:
  - rule: via_trace_clearance
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain via_trace_clearance` for the level this project gives it.
