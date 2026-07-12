# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project aims to
adhere to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Planned
- M2: RISC Zero receipt backend (`risc0` feature) and Rust→wasm TypeScript bindings.
- M3: EVM verifier contract with a worked testnet example.
- M4: spec v1.0 (audit-ready).

## [0.3.1] — 2026-07-12
### Added
- **`cost_floor` condition (SPEC §3.9).** Committed costs must meet the protocol
  floor (fee ≥ 7 bps, slippage ≥ 3 bps per position change). Up-only for provers —
  harsher costs strengthen a credential; understated costs are refused as a forge
  vector. Fee relaxation exists only out-of-band against an issuer-attested
  institutional fee schedule; the slippage floor is never waivable.
- Journal gains `fee_bps` / `slippage_bps` (absent → legacy default = the floor,
  since pre-disclosure journals were minted at the floor by construction).
- Vectors — pass vector now commits explicit harsher-than-floor costs; new fail
  vector: `cost_floor_understated`.

## [0.3.0] — 2026-07-12
### Added
- **Disclosure tier (SPEC §3.7–3.8).** Two opt-in, allocator-facing conditions:
  `exposure_disclosed` (v3) — in-circuit exposure card (net/gross bands, leverage rail,
  instrument count, long/short/flat bar accounting) derived from the same position series
  as the P&L, checked for internal consistency; `regime_pinned` (v4) — regime-conditional
  performance under the PINNED `vol30-trend100` policy (bucket-shopping is a rejection,
  not a choice; labels strictly causal).
- Journal schema: `window_bars`, optional `exposure_card`, optional `regime_panel` —
  all serde-defaulted, so v1/v2 journals deserialize unchanged and remain verifiable.
- New conformance vectors — pass: `v4_disclosing`; fail: `regime_policy_shopped`,
  `exposure_card_inconsistent`, `disclosure_required_but_minimal`.
- New rejection classes: `ExposureCardRequired/Inconsistent`,
  `RegimePanelRequired/Inconsistent`, `RegimePolicyNotPinned`.
### Changed
- `MAX_SUPPORTED_FORMAT_VERSION` 2 → 4; SPEC.md → v0.3; gate-policy JSON Schema enum extended.
### Notes
- The disclosure tier encodes transparency requirements stated directly by institutional
  allocators (exposure visibility; regime-conditional expectations) — provable without
  revealing the strategy. Base (output-minimized) credentials: 12 verifier conditions in
  the reference engine; disclosing credentials: 16. Disclosure is strictly opt-in.

## [0.2.0] — 2026-07-06
### Added
- **Credential format versioning.** Journal commits `format_version`; a verifier rejects a version it does not implement (fail-closed forward-incompatibility). SPEC.md §5.
- **`intrabar_risk` condition (v2).** Drawdown/worst-bar marked against the adverse intrabar extreme at leverage; a policy requiring it rejects a close-marked (`intrabar_marked = false`) journal.
- **`annualization_bound` condition (v2).** Annualization basis bound in-circuit to committed bar/print timestamps — closes a prover-chosen-periods Sharpe forge on both price and carry paths.
- New adversarial conformance vectors: `future_format_version`, `intrabar_required_but_close_marked`. New positive vector: `v1_close_marked` (backward compatibility).
### Changed
- Journal schema gains `format_version` (default 1) and `intrabar_marked` (default false) — v1 journals deserialize unchanged and remain verifiable under a v1 policy.
- SPEC.md → v0.2; gate-policy JSON Schema condition enum extended.
### Notes
- Conditions 5–6 were added after an adversarial review of the reference engine surfaced both as real soundness gaps; encoding them as normative conditions is intentional.

## [0.1.0] — 2026-06-30
### Added
- Gate-policy specification v0.1 (`SPEC.md`) and JSON Schema (`spec/gate_policy.schema.json`).
- Reference verifier library (`crates/verifier`): policy / journal / verdict logic, generic over a `ReceiptVerifier` backend.
- CLI (`crates/verify-cli`) with the `check-journal` subcommand.
- Conformance suite reading `vectors/`: positive vectors must `PASS`; adversarial vectors (lookahead-contaminated, strategy-revealed, policy-mismatch) must be rejected.
- EVM verifier contract stub (`contracts/`) and TypeScript binding surface (`bindings/ts/`).
- CI: fmt · clippy (`-D warnings`) · build · test.
- Project docs: README, CONTRIBUTING, SECURITY, CODE_OF_CONDUCT.

[Unreleased]: https://github.com/muaviamohammed/zk-strategy-verify/compare/v0.3.1...HEAD
[0.3.1]: https://github.com/muaviamohammed/zk-strategy-verify/releases/tag/v0.3.1
[0.3.0]: https://github.com/muaviamohammed/zk-strategy-verify/releases/tag/v0.3.0
[0.1.0]: https://github.com/muaviamohammed/zk-strategy-verify/releases/tag/v0.1.0
