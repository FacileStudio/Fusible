# `schematic_scale`

Schematic symbols stay inside a readable sheet.

| Field | Value |
| --- | --- |
| Category | erc |
| Default severity | warning |

## Change its level

```yaml
rules:
  schematic_scale: warning

waivers:
  - rule: schematic_scale
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain schematic_scale` for the level this project gives it.
