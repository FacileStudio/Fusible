# `power_undriven`

Every power input sits on a net that something supplies.

| Field | Value |
| --- | --- |
| Category | erc |
| Default severity | error |
| Standard | KiCad ERC power pin not driven |
| Source | https://gitlab.com/kicad/code/kicad/-/blob/10.0.3/eeschema/erc/erc.cpp |

## Change its level

```yaml
rules:
  power_undriven: warning

waivers:
  - rule: power_undriven
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain power_undriven` for the level this project gives it.
