# `nc_connected`

A pin declared `do_not_connect` shares its net with no other endpoint.

| Field | Value |
| --- | --- |
| Category | erc |
| Default severity | error |
| Standard | KiCad ERC pin conflicts map (no-connect row) |
| Source | https://gitlab.com/kicad/code/kicad/-/blob/10.0.3/eeschema/erc/erc_settings.cpp |

## Change its level

```yaml
rules:
  nc_connected: warning

waivers:
  - rule: nc_connected
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain nc_connected` for the level this project gives it.
