# `off_board`

tscircuit found a part outside the board outline.

| Field | Value |
| --- | --- |
| Category | upstream |
| Default severity | error |
| Standard | tscircuit |

## Change its level

```yaml
rules:
  off_board: warning

waivers:
  - rule: off_board
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain off_board` for the level this project gives it.
