# `placement`

tscircuit could not place a part legally.

| Field | Value |
| --- | --- |
| Category | upstream |
| Default severity | error |
| Standard | tscircuit |

## Change its level

```yaml
rules:
  placement: warning

waivers:
  - rule: placement
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain placement` for the level this project gives it.
