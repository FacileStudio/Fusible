# `mech.stale`

A body was rebuilt after its source last changed.

| Field | Value |
| --- | --- |
| Category | mechanical |
| Default severity | error |

## Change its level

```yaml
rules:
  mech.stale: warning

waivers:
  - rule: mech.stale
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain mech.stale` for the level this project gives it.
