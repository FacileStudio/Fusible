# `mech.not_built`

A declared body has been built.

| Field | Value |
| --- | --- |
| Category | mechanical |
| Default severity | error |

## Change its level

```yaml
rules:
  mech.not_built: warning

waivers:
  - rule: mech.not_built
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain mech.not_built` for the level this project gives it.
