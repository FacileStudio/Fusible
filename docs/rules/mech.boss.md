# `mech.boss`

Screw bosses are wide enough for their hole and bored for their insert.

| Field | Value |
| --- | --- |
| Category | mechanical |
| Default severity | error |

## Change its level

```yaml
rules:
  mech.boss: warning

waivers:
  - rule: mech.boss
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain mech.boss` for the level this project gives it.
