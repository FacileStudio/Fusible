# `pin_unconnected`

Every pin declared `must_be_connected`, and every power or ground input, is wired.

| Field | Value |
| --- | --- |
| Category | erc |
| Default severity | error |

## Change its level

```yaml
rules:
  pin_unconnected: warning

waivers:
  - rule: pin_unconnected
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain pin_unconnected` for the level this project gives it.
