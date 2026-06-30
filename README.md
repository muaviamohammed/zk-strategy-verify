# zk-strategy-verify

[![CI](https://github.com/muaviamohammed/zk-strategy-verify/actions/workflows/ci.yml/badge.svg)](https://github.com/muaviamohammed/zk-strategy-verify/actions/workflows/ci.yml)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](./LICENSE)
[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org)
[![Status: v0.1.0](https://img.shields.io/badge/status-v0.1.0_spec_draft-yellow.svg)](./SPEC.md)

An open standard and reference implementation for **verifying zero-knowledge proofs that a hidden systematic strategy satisfies a fixed set of integrity conditions** — without revealing the strategy.

A verifier *checks* a proof; it does not generate one and learns nothing about the strategy. This repository is the **verification half** of that system, released as a public good under Apache-2.0.

---

## Why

Generating a plausible-looking quantitative track record is now effectively free — an LLM can emit ten thousand perfect-looking backtests before lunch. On-chain asset managers, vaults, and RWA protocols increasingly need to gate capital on proof that a strategy meets fixed honesty conditions — **after costs, no lookahead, out-of-sample, walk-forward** — *without* forcing disclosure of the strategy. There is no shared, open standard to verify such a claim. This repository defines one.

## How it works

```
   PROVER  (out of scope, private)            VERIFIER  (this repository)
  ┌─────────────────────────────┐            ┌───────────────────────────────────┐
  │ strategy + canonical dataset │   STARK    │ 1. verify receipt  (RISC Zero)    │
  │   → zkVM guest program       │  receipt   │ 2. decode public journal          │
  │   → commit public journal    │ ─────────▶ │ 3. check journal vs gate policy   │
  │                              │            │ 4. → PASS · FAIL · REJECT         │
  └─────────────────────────────┘            └───────────────────────────────────┘
         the strategy stays secret                ~21 ms · off-chain or on the EVM
```

The verifier confirms (a) the receipt is cryptographically valid for a known guest image, and (b) the journal's public commitments satisfy a declared, versioned **gate policy**. It never sees the strategy. See [`SPEC.md`](./SPEC.md) for the normative specification and [`spec/gate_policy.schema.json`](./spec/gate_policy.schema.json) for the machine-checkable schema.

## What's here (and what's not)

**In scope (Apache-2.0):**
- [`spec/`](./spec) + [`SPEC.md`](./SPEC.md) — the versioned **gate-policy specification** + JSON Schema.
- [`crates/verifier/`](./crates/verifier) — **reference verifier library** (Rust): verifies a receipt and checks the journal against a declared policy. Generic over the cryptographic backend.
- [`crates/verify-cli/`](./crates/verify-cli) — a small **CLI** (`check-journal`).
- [`contracts/`](./contracts) — an **EVM verifier contract** (M3).
- [`bindings/ts/`](./bindings/ts) — **TypeScript/wasm** binding surface (M2).
- [`vectors/`](./vectors) — **conformance test vectors**, including adversarial negatives.

**Out of scope (intentionally):** any prover / proof-generation pipeline, strategy logic, or datasets. They are unnecessary for verification and are not part of this public good. A verifier checks proofs; it does not generate them.

## Try it

```bash
cargo test --all        # conformance suite: pass/ must PASS, adversarial fail/ must be rejected

# check a journal against a gate policy (SPEC.md §2-§3):
cargo run -p verify-cli -- check-journal examples/policy.json examples/journal_pass.json
#   PASS  gate_policy=gp_demo_v0_1 digest=0xclean
cargo run -p verify-cli -- check-journal examples/policy.json examples/journal_fail.json
#   FAIL  gate_policy=gp_demo_v0_1 digest=0xlookahead
```

The policy/journal conformance logic is implemented and tested today. Cryptographic receipt verification (the RISC Zero backend) is wired in at milestone **M2** behind the `risc0` feature — off by default so everything builds without a zkVM toolchain.

## The integrity conditions

A gate policy declares thresholds for, at minimum:

| Condition | Meaning |
|---|---|
| `after_costs` | returns are net of declared costs / fees / slippage |
| `no_lookahead` | signals at time *t* use only information available at *t* |
| `out_of_sample` | performance holds on a held-out segment not used for selection |
| `walk_forward` | performance holds under rolling re-fit / re-test |

The policy is versioned and extensible. The in-circuit evaluation over all conditions is committed to the journal as a single `verdict`; the verifier confirms it.

## Layout

```
crates/verifier      reference verifier library (error · journal · policy · verdict · receipt trait)
crates/verify-cli    CLI (check-journal)
contracts/           EVM verifier contract (M3)
bindings/ts/         TypeScript / wasm bindings (M2)
spec/  SPEC.md        gate-policy specification + JSON Schema
vectors/             conformance vectors (pass/ + adversarial fail/)
examples/            runnable policy + journals
```

## Roadmap

- [x] **M1** — gate-policy spec v0.1 · JSON Schema · adversarial vectors · policy/journal verifier · CLI · CI
- [ ] **M2** — RISC Zero receipt backend (`risc0` feature) · Rust→wasm TS bindings
- [ ] **M3** — EVM verifier contract · testnet example · gas + integration docs
- [ ] **M4** — spec v1.0 (audit-ready) · full docs

## Contributing

Issues and PRs welcome — especially adversarial test vectors. See [CONTRIBUTING.md](./CONTRIBUTING.md), [SECURITY.md](./SECURITY.md), and [CODE_OF_CONDUCT.md](./CODE_OF_CONDUCT.md).

## License

Apache-2.0. See [LICENSE](./LICENSE).

## Acknowledgements

Built on [RISC Zero](https://risczero.com/)'s zkVM. Cryptography review: Dr. Anish Mohammed.
