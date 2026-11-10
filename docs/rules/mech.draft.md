# `mech.draft`

A moulded body declares enough draft.

| Field | Value |
| --- | --- |
| Category | mechanical |
| Default severity | error |

## Change its level

```yaml
rules:
  mech.draft: warning

waivers:
  - rule: mech.draft
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain mech.draft` for the level this project gives it.
