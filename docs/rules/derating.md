# `derating`

Each capacitor is rated for at least 1.25 times the voltage of its rail.

| Field | Value |
| --- | --- |
| Category | erc |
| Default severity | error |
| Standard | voltage derating guideline |
| Source | https://industrialmonitordirect.com/blogs/knowledgebase/electrolytic-capacitor-voltage-rating-selection-guide |

## Change its level

```yaml
rules:
  derating: warning

waivers:
  - rule: derating
    match: U1.VDD
    reason: why this case is safe
```

Run `fusible explain derating` for the level this project gives it.
