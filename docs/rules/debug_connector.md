# `debug_connector`

The MCU has a connector or test point on a debug or programming pin.

| Field | Value |
| --- | --- |
| Category | erc |
| Default severity | warning |

## Change its level

```yaml
rules:
  debug_connector: warning

waivers:
  - rule: debug_connector
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain debug_connector` for the level this project gives it.
