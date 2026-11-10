# `mech.standoff_height`

Standoffs are taller than the deepest part under the board.

| Field | Value |
| --- | --- |
| Category | mechanical |
| Default severity | error |

## Change its level

```yaml
rules:
  mech.standoff_height: warning

waivers:
  - rule: mech.standoff_height
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain mech.standoff_height` for the level this project gives it.
