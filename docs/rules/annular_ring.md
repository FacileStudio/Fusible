# `annular_ring`

Plated holes and vias keep the fab's minimum copper ring around the drill.

| Field | Value |
| --- | --- |
| Category | dfm |
| Default severity | error |
| Standard | fab capability profile |

## Change its level

```yaml
rules:
  annular_ring: warning

waivers:
  - rule: annular_ring
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain annular_ring` for the level this project gives it.
