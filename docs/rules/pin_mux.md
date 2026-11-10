# `pin_mux`

Pin assignments respect the alternate functions of the target MCU.

| Field | Value |
| --- | --- |
| Category | firmware |
| Default severity | error |

## Change its level

```yaml
rules:
  pin_mux: warning

waivers:
  - rule: pin_mux
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain pin_mux` for the level this project gives it.
