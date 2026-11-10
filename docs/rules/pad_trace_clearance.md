# `pad_trace_clearance`

tscircuit found a trace too close to a pad.

| Field | Value |
| --- | --- |
| Category | upstream |
| Default severity | error |
| Standard | tscircuit |

## Change its level

```yaml
rules:
  pad_trace_clearance: warning

waivers:
  - rule: pad_trace_clearance
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain pad_trace_clearance` for the level this project gives it.
