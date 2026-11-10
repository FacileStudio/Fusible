# `voltage_domain`

Pins sit on nets of the voltage they need, and no output drives a lower-voltage input.

| Field | Value |
| --- | --- |
| Category | erc |
| Default severity | error |

## Change its level

```yaml
rules:
  voltage_domain: warning

waivers:
  - rule: voltage_domain
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain voltage_domain` for the level this project gives it.
