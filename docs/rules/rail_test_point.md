# `rail_test_point`

Each supply rail has a test point or connector pin to probe it on.

| Field | Value |
| --- | --- |
| Category | erc |
| Default severity | warning |

## Change its level

```yaml
rules:
  rail_test_point: warning

waivers:
  - rule: rail_test_point
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain rail_test_point` for the level this project gives it.
