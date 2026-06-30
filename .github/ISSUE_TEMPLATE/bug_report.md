---
name: Bug report
about: Report incorrect verifier behavior
title: "[bug] "
labels: bug
---

## Summary
<!-- One sentence. -->

## Reproduction
The most useful report is a test vector (a `policy` + `journal` pair):
```json
{ "policy": { }, "journal": { } }
```

## Expected vs actual verdict
- Expected: `PASS` / `FAIL` / `REJECT`
- Actual: `PASS` / `FAIL` / `REJECT`

## Environment
- crate version / commit:
- rust version (`rustc --version`):

> If this is a soundness break (a non-conforming journal verifying as PASS), please follow SECURITY.md instead of filing a public bug.
