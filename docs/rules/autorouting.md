# `autorouting`

tscircuit's autorouter failed or was skipped.

| Field | Value |
| --- | --- |
| Category | upstream |
| Default severity | error |
| Standard | tscircuit |

## Change its level

```yaml
rules:
  autorouting: warning

waivers:
  - rule: autorouting
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain autorouting` for the level this project gives it.
