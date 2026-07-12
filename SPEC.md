# Gate-Policy Specification — v0.3 (draft)

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
| `format_version` | Credential format version committed in-circuit (see §5) | MUST be ≤ the verifier's supported version, else REJECT |
| `intrabar_marked` | (v2) Whether risk was marked against intrabar extremes vs. close-only | Surfaced; MUST be true if the policy requires `intrabar_risk` |
| `window_bars` | (v3) Bars in the committed evaluation window | Required (> 0) when any disclosure-tier condition is required |
| `exposure_card` | (v3, disclosure tier, optional) In-circuit exposure accounting: net/gross bands, leverage rail, instrument count, long/short/flat bar accounting | If the policy requires `exposure_disclosed`: MUST be present and internally consistent (§3.7) |
| `regime_panel` | (v4, disclosure tier, optional) Per-regime mean return + bar counts under the PINNED regime policy | If the policy requires `regime_pinned`: MUST be present, under the pinned policy, and consistent (§3.8) |
| `digest` | Overall commitment binding the above | MUST be consistent |

A journal that omits `format_version` / `intrabar_marked` is a **v1** journal: it deserializes to `format_version = 1`, `intrabar_marked = false`, and is accepted under a v1 policy. v1 and v2 data commitments never collide (distinct Merkle-leaf domains), so a v1 credential can never masquerade as v2.

## 3. Integrity conditions (evaluated in-circuit, summarized to `verdict`)
A conforming gate policy declares thresholds for, at minimum:
1. **after_costs** — returns are net of declared costs/fees/slippage.
2. **no_lookahead** — signals at time *t* use only information available at *t* (causal evaluation).
3. **out_of_sample** — performance holds on a held-out segment not used for selection.
4. **walk_forward** — performance holds under rolling re-fit/re-test.
5. **intrabar_risk** *(v2)* — drawdown and worst-bar are marked against the **adverse intrabar extreme** at the position's leverage, not close-to-close. A levered wick that nearly liquidates but recovers by the close is therefore visible to the gate, not hidden. Requires OHLC data; a policy that requires this condition MUST reject a journal with `intrabar_marked = false`.
6. **annualization_bound** *(v2)* — the annualization basis (periods-per-year used to scale Sharpe and CAGR) is **bound in-circuit to the committed bar/print timestamps**. A prover cannot declare more periods per year than the data's spacing implies to inflate a passing Sharpe (√-of-the-ratio forge). Covers both the price path (bar interval) and the timestamped-funding carry path.
7. **exposure_disclosed** *(v3, disclosure tier)* — the journal carries an **exposure card** derived in-circuit from the *same* position × leverage series that produced the P&L: average/min/max net exposure, average/max gross exposure, the declared leverage rail, instrument count, and long/short/flat bar accounting. The verifier checks internal consistency: bar accounting covers the committed window exactly, bands are ordered, gross never exceeds the rail, and |net| never exceeds gross. Answers the allocator's *"what am I actually exposed to?"* without revealing the strategy.
8. **regime_pinned** *(v4, disclosure tier)* — the journal carries **regime-conditional performance** (mean per-bar return + bar count per regime) under a **pinned** regime policy: 30-bar trailing-vol terciles × 100-bar trend state (`vol30-trend100`), labels strictly causal. Pinning is normative — a prover cannot choose the bucketing that flatters the strategy; a journal claiming any other regime policy MUST be rejected. A full verifier additionally re-derives the bucket bar-counts from its own copy of the public dataset; the reference implementation checks structure + pinning. Answers *"what should I expect from this strategy in the current regime?"*

**The disclosure tier.** Conditions 1–6 are the **base tier**: every conforming credential commits them, and an output-minimized credential reveals nothing beyond PASS/FAIL. Conditions 7–8 are the **disclosure tier**: opt-in fields the quant may commit for allocator-grade transparency (in the reference engine, a base credential is checked on 12 verifier conditions and a disclosing credential on 16). Disclosure never weakens privacy involuntarily — a minimal journal remains fully verifiable under any base policy.

*(Additional conditions MAY be declared; the policy is versioned and extensible.)*

Conditions 7–8 encode the transparency requirements institutional allocators stated directly (exposure visibility; regime-conditional expectations) — proved without opening the strategy. Conditions 5–6 were added in v0.2 after an adversarial review of the reference engine found both as real soundness gaps — a close-only drawdown that hid a −16% intrabar wick, and a prover-chosen annualization factor. Publishing them as normative conditions is deliberate: the spec should encode the failures a conforming verifier must refuse, not only the happy path. See the project's published security review.

The verifier does **not** re-run the strategy. It confirms the receipt proves the in-circuit evaluation of the declared policy returned PASS over the committed dataset.

## 4. Conformance
An implementation is conforming if it (a) verifies all positive test vectors in `vectors/pass/`, and (b) rejects all adversarial vectors in `vectors/fail/` (e.g. lookahead-contaminated, pre-cost, in-sample-only). Negative vectors are normative: a verifier that accepts any `fail/` vector is non-conforming.

## 5. Versioning
This spec is versioned (`v0.1` → `v0.2` → `v0.3` → `v1.0`). Format versions: `1` = close-marked risk; `2` adds intrabar marking + the bound annualization basis; `3` adds the exposure card (disclosure tier); `4` adds the pinned regime panel (disclosure tier). `gate_policy` commits to the exact policy+version evaluated, and the journal commits `format_version`, so receipts are unambiguous across revisions. A conforming verifier MUST **reject** a journal whose `format_version` exceeds the version it implements, rather than risk misreading a newer schema — forward-incompatibility is fail-closed. A v1 credential remains verifiable by a v1 verifier; the reference implementation tags the v1 release for exactly this.

---
*v0.3 adds the disclosure tier (exposure card + pinned regime panel) — the allocator-transparency conditions the reference engine ships as credential format v3/v4. v0.2 folded in the intrabar-risk and annualization-binding conditions surfaced by the reference engine's security review. The spec will be refined further through EF Office Hours feedback toward v1.0 (audit-ready).*
