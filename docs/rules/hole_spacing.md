# `hole_spacing`

Holes keep the fab's minimum drill edge to drill edge spacing.

| Field | Value |
| --- | --- |
| Category | dfm |
| Default severity | error |
| Standard | fab capability profile |

## Change its level

```yaml
rules:
  hole_spacing: warning

waivers:
  - rule: hole_spacing
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain hole_spacing` for the level this project gives it.
