# `edge_clearance`

Copper keeps the fab's minimum distance to the routed board edge.

| Field | Value |
| --- | --- |
| Category | dfm |
| Default severity | error |
| Standard | fab capability profile |

## Change its level

```yaml
rules:
  edge_clearance: warning

waivers:
  - rule: edge_clearance
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain edge_clearance` for the level this project gives it.
