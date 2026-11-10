# `fab_profile`

Trace width, spacing, drills, vias and board size stay within the declared fab limits.

| Field | Value |
| --- | --- |
| Category | dfm |
| Default severity | error |
| Standard | fab capability profile |

## Change its level

```yaml
rules:
  fab_profile: warning

waivers:
  - rule: fab_profile
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain fab_profile` for the level this project gives it.
