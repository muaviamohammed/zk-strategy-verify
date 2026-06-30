# zk-strategy-verify

An open standard and reference implementation for **verifying zero-knowledge proofs that a hidden systematic strategy satisfies a fixed set of integrity conditions** — without revealing the strategy.

A verifier *checks* a proof; it does not generate one and learns nothing about the strategy. This repository is the **verification half** of that system, released as a public good.

## Why
Generating a plausible-looking quantitative track record is now effectively free. On-chain asset managers, vaults, and RWA protocols increasingly need to gate capital on proof that a strategy meets fixed honesty conditions — **after costs, no lookahead, out-of-sample, walk-forward** — without forcing disclosure of the strategy. There is no shared, open standard to verify such a claim. This repo defines one.

## What's here (and what's not)
**In scope (Apache-2.0):**
- `spec/` — the versioned **gate-policy specification** + JSON schema for the conditions committed in a proof journal.
- `verifier/` — a **reference verifier library** (Rust + TS bindings): verifies a RISC Zero STARK receipt and checks the journal commitments against a declared gate policy. Returns a typed verdict.
- `contracts/` — an **EVM verifier contract**: a thin wrapper over the on-chain proof verifier that additionally enforces the gate-policy commitments.
- `vectors/` — **conformance test vectors**, including adversarial negatives (a lookahead-contaminated or post-cost-failing strategy MUST fail).

**Out of scope (intentionally):** any prover / proof-generation pipeline, strategy logic, or datasets. They are unnecessary for verification and are not part of this public good.

## The integrity conditions (gate policy)
A proof commits, in its public journal, to satisfying a declared policy over conditions including:
`image_id · receipt · journal · data_root · gate_policy · dataset_canonicalization · verdict · strategy_hidden · digest`
The verifier confirms the cryptographic validity of the receipt **and** that the committed values satisfy the declared policy. See `spec/`.

## Status
🚧 Early. Spec v0.1 and conformance vectors land first (milestone M1). Reference verifier (M2), EVM contract (M3), spec v1.0 (M4).

## License
Apache-2.0. See [LICENSE](./LICENSE).

## Acknowledgements
Built on [RISC Zero](https://risczero.com/)'s zkVM. Cryptography review: Dr. Anish Mohammed.
