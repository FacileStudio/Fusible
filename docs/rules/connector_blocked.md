# `connector_blocked`

A connector's opening is clear of body material.

| Field | Value |
| --- | --- |
| Category | mechanical |
| Default severity | error |

## Change its level

```yaml
rules:
  connector_blocked: warning

waivers:
  - rule: connector_blocked
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain connector_blocked` for the level this project gives it.
