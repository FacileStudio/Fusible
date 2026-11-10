# `led_series_resistor`

An LED on a supply rail has a series resistor to limit its current.

| Field | Value |
| --- | --- |
| Category | erc |
| Default severity | warning |

## Change its level

```yaml
rules:
  led_series_resistor: warning

waivers:
  - rule: led_series_resistor
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain led_series_resistor` for the level this project gives it.
