# `silk_text_size`

Silkscreen text and lines meet the fab's minimum height and stroke.

| Field | Value |
| --- | --- |
| Category | dfm |
| Default severity | warning |
| Standard | fab capability profile |

## Change its level

```yaml
rules:
  silk_text_size: warning

waivers:
  - rule: silk_text_size
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain silk_text_size` for the level this project gives it.
