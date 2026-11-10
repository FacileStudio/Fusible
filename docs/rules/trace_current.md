# `trace_current`

A net that declares a current must not heat its narrowest trace past its allowed rise.

| Field | Value |
| --- | --- |
| Category | drc |
| Default severity | error |
| Standard | IPC-2152 |
| Source | https://docs.kicad.org/doxygen/tracks__width__versus__current__formula_8h.html |

## Change its level

```yaml
rules:
  trace_current: warning

waivers:
  - rule: trace_current
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain trace_current` for the level this project gives it.
