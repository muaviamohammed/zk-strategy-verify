# Contributing

This is a public good. The goal is one open, conformant standard for verifying — in zero knowledge — that a hidden systematic strategy satisfies a fixed set of integrity conditions (after-costs, no-lookahead, out-of-sample, walk-forward). Issues, PRs, and review are all welcome.

## Where help matters most
- **Conformance test vectors** — especially adversarial `fail/` cases. A vector that *should* be rejected but currently isn't is a high-value contribution. See `vectors/`.
- **Spec review** — ambiguities, under-specified commitments, or conditions worth adding. See `SPEC.md`.
- **Reference verifier** — correctness, clarity, and the M2 RISC Zero backend (`crates/verifier`, the `risc0` feature).
- **EVM verifier** — gas, integration ergonomics, the M3 wiring (`contracts/`).

## Ground rules
- **The prover is out of scope.** This repository is the *verification* half only. PRs that add proof-generation, strategy logic, or datasets will be declined — a verifier checks proofs, it does not generate them. (See the Scope note in `README.md`.)
- **Negative vectors are normative.** A change that causes any `vectors/fail/` case to be accepted as `PASS` is a regression, not a feature.
- **Keep it falsifiable.** New conditions must come with both a positive and an adversarial vector.

## Dev workflow
```bash
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo test --all        # the conformance suite must stay green
```
CI runs fmt · clippy · build · test on every PR. Please make sure those pass locally first.

## PRs
- One focused change per PR; describe *what* and *why*.
- Add/extend test vectors for any behavior change.
- By contributing you agree your contribution is licensed under **Apache-2.0** (the repository license).

## Reporting issues
Open an issue with a minimal reproduction. For a verifier bug, the most useful report is a **test vector** (policy + journal) plus the verdict you got vs. the verdict you expected.

## Security
The verifier checks proofs; it never sees a strategy. If you find a way to make a non-conforming journal verify as `PASS` (a soundness break), please open an issue marked **security** with a reproducing vector rather than a public PR first.
