# `mech.vent_width`

Vent slots are at least as wide as the process allows.

| Field | Value |
| --- | --- |
| Category | mechanical |
| Default severity | error |

## Change its level

```yaml
rules:
  mech.vent_width: warning

waivers:
  - rule: mech.vent_width
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain mech.vent_width` for the level this project gives it.
