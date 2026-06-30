# Security Policy

The verifier checks proofs; it never sees a strategy. The security property that matters here is **soundness**: a non-conforming journal must never verify as `PASS`.

## Reporting a vulnerability

If you find a way to make a non-conforming proof or journal verify as `PASS` (a soundness break), or any other security issue, please **do not open a public PR first**.

- Open a GitHub issue labelled **security** with a minimal reproduction, **or**
- email **muavia@mizan.market**.

The most useful report is a **test vector** (a `policy` + `journal` pair) plus the verdict you got versus the verdict you expected.

## Scope

In scope: the gate-policy spec, the reference verifier, the EVM verifier contract, and the conformance vectors. Out of scope: any prover / proof-generation pipeline (not part of this repository).

## Disclosure

We aim to acknowledge reports within 5 business days and to coordinate a fix and disclosure timeline with the reporter.
