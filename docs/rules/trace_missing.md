# `trace_missing`

tscircuit found a connection with no PCB trace.

| Field | Value |
| --- | --- |
| Category | upstream |
| Default severity | error |
| Standard | tscircuit |

## Change its level

```yaml
rules:
  trace_missing: warning

waivers:
  - rule: trace_missing
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain trace_missing` for the level this project gives it.
