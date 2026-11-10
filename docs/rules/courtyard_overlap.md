# `courtyard_overlap`

tscircuit found two part courtyards overlapping.

| Field | Value |
| --- | --- |
| Category | upstream |
| Default severity | error |
| Standard | tscircuit |

## Change its level

```yaml
rules:
  courtyard_overlap: warning

waivers:
  - rule: courtyard_overlap
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain courtyard_overlap` for the level this project gives it.
