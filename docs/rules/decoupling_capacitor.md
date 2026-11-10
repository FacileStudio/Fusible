# `decoupling_capacitor`

Each IC power pin needs a bypass capacitor on its rail, placed within 3 mm of the pin.

| Field | Value |
| --- | --- |
| Category | erc |
| Default severity | error |

## Change its level

```yaml
rules:
  decoupling_capacitor: warning

waivers:
  - rule: decoupling_capacitor
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain decoupling_capacitor` for the level this project gives it.
