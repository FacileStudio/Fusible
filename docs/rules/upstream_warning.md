# `upstream_warning`

tscircuit reported a warning no other rule names.

| Field | Value |
| --- | --- |
| Category | upstream |
| Default severity | warning |
| Standard | tscircuit |

## Change its level

```yaml
rules:
  upstream_warning: warning

waivers:
  - rule: upstream_warning
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain upstream_warning` for the level this project gives it.
