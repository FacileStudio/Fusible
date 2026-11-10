# `silk_over_pad`

Silkscreen keeps the fab's minimum distance to pad copper.

| Field | Value |
| --- | --- |
| Category | dfm |
| Default severity | warning |
| Standard | fab capability profile |

## Change its level

```yaml
rules:
  silk_over_pad: warning

waivers:
  - rule: silk_over_pad
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain silk_over_pad` for the level this project gives it.
