# Gate-Policy Specification — v0.1 (draft)

Status: **draft / pre-implementation.** This document defines the public commitments a conforming integrity-proof must expose, and the checks a conforming verifier must perform. It is the normative reference for the verifier library and the EVM contract.

## 1. Model
A *prover* (out of scope for this repo) runs a systematic strategy over a canonical dataset inside a zkVM guest program and emits a STARK receipt. The receipt's public **journal** commits to a set of values. A *verifier* (in scope) checks (a) the receipt is cryptographically valid for a known guest image, and (b) the journal's committed values satisfy a declared **gate policy**. The verifier never sees the strategy.

## 2. Committed fields (journal)
| Field | Meaning | Verifier check |
|---|---|---|
| `image_id` | Identifier of the guest program that produced the proof | MUST match an allowed image in the policy |
| `receipt` | The STARK receipt (proof object) | MUST verify against `image_id` |
| `journal` | The committed public outputs | MUST decode to this schema |
| `data_root` | Commitment (e.g. Merkle root) to the canonical dataset used | MUST match the declared dataset commitment |
| `gate_policy` | Identifier/hash of the policy evaluated in-circuit | MUST equal the policy the verifier is checking against |
| `dataset_canonicalization` | Commitment to the canonicalization rules (timezone, adjustments, point-in-time) | MUST match policy |
| `verdict` | The in-circuit pass/fail over all conditions | MUST be PASS for acceptance |
| `strategy_hidden` | Assertion that no strategy parameters are revealed in the journal | MUST be true |
| `digest` | Overall commitment binding the above | MUST be consistent |

## 3. Integrity conditions (evaluated in-circuit, summarized to `verdict`)
A conforming gate policy declares thresholds for, at minimum:
1. **after_costs** — returns are net of declared costs/fees/slippage.
2. **no_lookahead** — signals at time *t* use only information available at *t* (causal evaluation).
3. **out_of_sample** — performance holds on a held-out segment not used for selection.
4. **walk_forward** — performance holds under rolling re-fit/re-test.
*(Additional conditions MAY be declared; the policy is versioned and extensible.)*

The verifier does **not** re-run the strategy. It confirms the receipt proves the in-circuit evaluation of the declared policy returned PASS over the committed dataset.

## 4. Conformance
An implementation is conforming if it (a) verifies all positive test vectors in `vectors/pass/`, and (b) rejects all adversarial vectors in `vectors/fail/` (e.g. lookahead-contaminated, pre-cost, in-sample-only). Negative vectors are normative: a verifier that accepts any `fail/` vector is non-conforming.

## 5. Versioning
This spec is versioned (`v0.1` → `v1.0`). `gate_policy` commits to the exact policy+version evaluated, so receipts are unambiguous across spec revisions.

---
*v0.1 is intentionally minimal and will be refined through EF Office Hours feedback and milestone M1.*
