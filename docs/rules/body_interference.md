# `body_interference`

Two bodies, or a body and a board, never share volume.

| Field | Value |
| --- | --- |
| Category | mechanical |
| Default severity | error |

## Change its level

```yaml
rules:
  body_interference: warning

waivers:
  - rule: body_interference
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain body_interference` for the level this project gives it.
