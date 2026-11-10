# `acid_trap`

Consecutive trace segments meet at 90 degrees or more, leaving no acid trap.

| Field | Value |
| --- | --- |
| Category | dfm |
| Default severity | warning |

## Change its level

```yaml
rules:
  acid_trap: warning

waivers:
  - rule: acid_trap
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain acid_trap` for the level this project gives it.
