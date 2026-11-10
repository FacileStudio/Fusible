# `mech.wall_thickness`

Walls are thick enough for the process, and not so thick that a moulding sinks.

| Field | Value |
| --- | --- |
| Category | mechanical |
| Default severity | error |

## Change its level

```yaml
rules:
  mech.wall_thickness: warning

waivers:
  - rule: mech.wall_thickness
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain mech.wall_thickness` for the level this project gives it.
