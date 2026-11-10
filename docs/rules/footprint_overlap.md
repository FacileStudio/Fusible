# `footprint_overlap`

tscircuit found two footprints overlapping.

| Field | Value |
| --- | --- |
| Category | upstream |
| Default severity | error |
| Standard | tscircuit |

## Change its level

```yaml
rules:
  footprint_overlap: warning

waivers:
  - rule: footprint_overlap
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain footprint_overlap` for the level this project gives it.
