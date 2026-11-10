# `unnamed_trace`

tscircuit found a trace without a name; informational.

| Field | Value |
| --- | --- |
| Category | upstream |
| Default severity | info |
| Standard | tscircuit |

## Change its level

```yaml
rules:
  unnamed_trace: warning

waivers:
  - rule: unnamed_trace
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain unnamed_trace` for the level this project gives it.
