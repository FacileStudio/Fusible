# `bom_incomplete`

Every placed part has a part number on the component or in bom.yml.

| Field | Value |
| --- | --- |
| Category | dfa |
| Default severity | warning |

## Change its level

```yaml
rules:
  bom_incomplete: warning

waivers:
  - rule: bom_incomplete
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain bom_incomplete` for the level this project gives it.
