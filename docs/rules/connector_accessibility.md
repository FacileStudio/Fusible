# `connector_accessibility`

tscircuit found a connector that cannot be reached.

| Field | Value |
| --- | --- |
| Category | upstream |
| Default severity | error |
| Standard | tscircuit |

## Change its level

```yaml
rules:
  connector_accessibility: warning

waivers:
  - rule: connector_accessibility
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain connector_accessibility` for the level this project gives it.
