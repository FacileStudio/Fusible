# `upstream_error`

tscircuit reported an error no other rule names.

| Field | Value |
| --- | --- |
| Category | upstream |
| Default severity | error |
| Standard | tscircuit |

## Change its level

```yaml
rules:
  upstream_error: warning

waivers:
  - rule: upstream_error
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain upstream_error` for the level this project gives it.
