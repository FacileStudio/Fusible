# `mask_sliver`

The solder mask web between neighbouring pads meets the fab's minimum dam.

| Field | Value |
| --- | --- |
| Category | dfm |
| Default severity | warning |
| Standard | fab capability profile |

## Change its level

```yaml
rules:
  mask_sliver: warning

waivers:
  - rule: mask_sliver
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain mask_sliver` for the level this project gives it.
