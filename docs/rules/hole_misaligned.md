# `hole_misaligned`

Every non-plated hole of a fitted board meets a boss and an empty bore.

| Field | Value |
| --- | --- |
| Category | mechanical |
| Default severity | error |

## Change its level

```yaml
rules:
  hole_misaligned: warning

waivers:
  - rule: hole_misaligned
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain hole_misaligned` for the level this project gives it.
