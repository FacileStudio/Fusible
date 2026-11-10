# `harness_mismatch`

Both ends of a `links:` wire name an existing part, pin and net.

| Field | Value |
| --- | --- |
| Category | erc |
| Default severity | error |

## Change its level

```yaml
rules:
  harness_mismatch: warning

waivers:
  - rule: harness_mismatch
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain harness_mismatch` for the level this project gives it.
