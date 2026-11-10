# `port_not_connected`

tscircuit found ports of one trace on different nets.

| Field | Value |
| --- | --- |
| Category | upstream |
| Default severity | error |
| Standard | tscircuit |

## Change its level

```yaml
rules:
  port_not_connected: warning

waivers:
  - rule: port_not_connected
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain port_not_connected` for the level this project gives it.
