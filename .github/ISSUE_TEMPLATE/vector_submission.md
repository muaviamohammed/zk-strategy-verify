---
name: Conformance vector
about: Propose a new conformance test vector (especially an adversarial one)
title: "[vector] "
labels: vector
---

## What this vector checks
<!-- e.g. "a strategy that fails the after_costs condition must be rejected" -->

## Expected outcome
- [ ] should verify to `PASS` (belongs in `vectors/pass/`)
- [ ] should be rejected — `FAIL` verdict or `Err` (belongs in `vectors/fail/`)

## The vector
```json
{
  "policy": { },
  "journal": { }
}
```

## Why it matters
<!-- What gap in coverage does this close? Adversarial vectors are especially valued. -->
