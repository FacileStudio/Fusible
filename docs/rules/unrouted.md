# `unrouted`

Blows the fuse when a board has nets but no routed trace, or copper does not reach every port of a net.

| Field | Value |
| --- | --- |
| Category | drc |
| Default severity | error |
| Standard | tscircuit |

## Change its level

```yaml
rules:
  unrouted: warning

waivers:
  - rule: unrouted
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain unrouted` for the level this project gives it.
