# `pad_pad_clearance`

tscircuit found two pads closer than allowed.

| Field | Value |
| --- | --- |
| Category | upstream |
| Default severity | error |
| Standard | tscircuit |

## Change its level

```yaml
rules:
  pad_pad_clearance: warning

waivers:
  - rule: pad_pad_clearance
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain pad_pad_clearance` for the level this project gives it.
