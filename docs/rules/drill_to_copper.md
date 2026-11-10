# `drill_to_copper`

Every drill keeps the fab's minimum distance to copper of other nets.

| Field | Value |
| --- | --- |
| Category | dfm |
| Default severity | error |
| Standard | fab capability profile |

## Change its level

```yaml
rules:
  drill_to_copper: warning

waivers:
  - rule: drill_to_copper
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain drill_to_copper` for the level this project gives it.
