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

## Try it
```bash
cargo test --all        # runs the conformance suite (pass/ must PASS, fail/ must be rejected)

# check a journal against a gate policy (SPEC.md §2-§3):
cargo run -p verify-cli -- check-journal examples/policy.json examples/journal_pass.json
#   PASS  gate_policy=gp_demo_v0_1 digest=0xclean
cargo run -p verify-cli -- check-journal examples/policy.json examples/journal_fail.json
#   FAIL  gate_policy=gp_demo_v0_1 digest=0xlookahead
```
The policy/journal conformance logic is implemented and tested today. Cryptographic
receipt verification (the RISC Zero backend) is wired in at milestone M2 behind the
`risc0` feature — kept off by default so everything builds without a zkVM toolchain.

## Layout
```
crates/verifier      reference verifier library (policy · journal · verdict · receipt trait)
crates/verify-cli    CLI (check-journal subcommand)
contracts/           EVM verifier contract (M3)
bindings/ts/         TypeScript/wasm bindings (M2)
spec/  SPEC.md        gate-policy specification
vectors/             conformance vectors (pass/ + adversarial fail/)
```

## Status
🚧 Early. M1: spec v0.1 + conformance vectors + policy/journal verifier (**done in this skeleton**). M2: RISC Zero receipt backend + TS/wasm. M3: EVM contract. M4: spec v1.0.

## License
Apache-2.0. See [LICENSE](./LICENSE).

## Acknowledgements
Built on [RISC Zero](https://risczero.com/)'s zkVM. Cryptography review: Dr. Anish Mohammed.
