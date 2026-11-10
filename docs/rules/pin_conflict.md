# `pin_conflict`

No net joins two pins that drive against each other, such as two push-pull outputs.

| Field | Value |
| --- | --- |
| Category | erc |
| Default severity | error |
| Standard | KiCad ERC pin conflicts map |
| Source | https://gitlab.com/kicad/code/kicad/-/blob/10.0.3/eeschema/erc/erc_settings.cpp |

## Change its level

```yaml
rules:
  pin_conflict: warning

waivers:
  - rule: pin_conflict
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain pin_conflict` for the level this project gives it.
