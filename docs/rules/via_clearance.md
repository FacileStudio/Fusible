# `via_clearance`

tscircuit found two vias closer than allowed.

| Field | Value |
| --- | --- |
| Category | upstream |
| Default severity | error |
| Standard | tscircuit |

## Change its level

```yaml
rules:
  via_clearance: warning

waivers:
  - rule: via_clearance
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain via_clearance` for the level this project gives it.
