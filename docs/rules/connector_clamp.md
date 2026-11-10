# `connector_clamp`

A connector pin wired to an MCU GPIO has a clamp diode on its net.

| Field | Value |
| --- | --- |
| Category | erc |
| Default severity | warning |

## Change its level

```yaml
rules:
  connector_clamp: warning

waivers:
  - rule: connector_clamp
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain connector_clamp` for the level this project gives it.
