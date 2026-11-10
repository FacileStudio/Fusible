# `rail_budget`

The loads declared on a rail stay within the current its source can deliver.

| Field | Value |
| --- | --- |
| Category | erc |
| Default severity | error |

## Change its level

```yaml
rules:
  rail_budget: warning

waivers:
  - rule: rail_budget
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain rail_budget` for the level this project gives it.
