# `qfn_footprint_span`

Warns when a qfn footprint gives only a copper span, which can short the corner pads.

| Field | Value |
| --- | --- |
| Category | dfm |
| Default severity | warning |
| Standard | tscircuit footprinter |

## Change its level

```yaml
rules:
  qfn_footprint_span: warning

waivers:
  - rule: qfn_footprint_span
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain qfn_footprint_span` for the level this project gives it.
