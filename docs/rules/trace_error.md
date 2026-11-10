# `trace_error`

tscircuit found a faulty routed trace.

| Field | Value |
| --- | --- |
| Category | upstream |
| Default severity | error |
| Standard | tscircuit |

## Change its level

```yaml
rules:
  trace_error: warning

waivers:
  - rule: trace_error
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain trace_error` for the level this project gives it.
