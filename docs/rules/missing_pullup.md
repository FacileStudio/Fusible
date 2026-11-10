# `missing_pullup`

Open-drain nets and pins that need an external pull-up or pull-down have a resistor to the rail.

| Field | Value |
| --- | --- |
| Category | erc |
| Default severity | error |

## Change its level

```yaml
rules:
  missing_pullup: warning

waivers:
  - rule: missing_pullup
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain missing_pullup` for the level this project gives it.
