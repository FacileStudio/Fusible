# `via_aspect_ratio`

Plated drills stay within the fab's maximum board thickness to drill ratio.

| Field | Value |
| --- | --- |
| Category | dfm |
| Default severity | error |
| Standard | fab capability profile |

## Change its level

```yaml
rules:
  via_aspect_ratio: warning

waivers:
  - rule: via_aspect_ratio
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain via_aspect_ratio` for the level this project gives it.
