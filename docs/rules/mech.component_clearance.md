# `mech.component_clearance`

A body keeps its gap around the board and the parts inside it.

| Field | Value |
| --- | --- |
| Category | mechanical |
| Default severity | error |

## Change its level

```yaml
rules:
  mech.component_clearance: warning

waivers:
  - rule: mech.component_clearance
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain mech.component_clearance` for the level this project gives it.
