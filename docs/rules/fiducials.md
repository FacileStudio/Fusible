# `fiducials`

With assembly_service set, every side with SMT pads has at least two fiducials.

| Field | Value |
| --- | --- |
| Category | dfa |
| Default severity | warning |
| Standard | IPC-7351B |

## Change its level

```yaml
rules:
  fiducials: warning

waivers:
  - rule: fiducials
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain fiducials` for the level this project gives it.
