# `component_edge_clearance`

Part courtyards keep the fab's minimum distance to the board edge.

| Field | Value |
| --- | --- |
| Category | dfa |
| Default severity | warning |
| Standard | fab capability profile |

## Change its level

```yaml
rules:
  component_edge_clearance: warning

waivers:
  - rule: component_edge_clearance
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain component_edge_clearance` for the level this project gives it.
