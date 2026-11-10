# `mech.port_clearance`

A connector cutout leaves the margin the process needs on each side.

| Field | Value |
| --- | --- |
| Category | mechanical |
| Default severity | error |

## Change its level

```yaml
rules:
  mech.port_clearance: warning

waivers:
  - rule: mech.port_clearance
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain mech.port_clearance` for the level this project gives it.
