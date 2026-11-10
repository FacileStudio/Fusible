# `courtyard_clearance`

Part courtyards, turned with their parts, do not overlap on one side.

| Field | Value |
| --- | --- |
| Category | dfa |
| Default severity | error |
| Standard | IPC-7351B |

## Change its level

```yaml
rules:
  courtyard_clearance: warning

waivers:
  - rule: courtyard_clearance
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain courtyard_clearance` for the level this project gives it.
