# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project aims to
adhere to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Planned
- M2: RISC Zero receipt backend (`risc0` feature) and Rust→wasm TypeScript bindings.
- M3: EVM verifier contract with a worked testnet example.
- M4: spec v1.0 (audit-ready).

## [0.1.0] — 2026-06-30
### Added
- Gate-policy specification v0.1 (`SPEC.md`) and JSON Schema (`spec/gate_policy.schema.json`).
- Reference verifier library (`crates/verifier`): policy / journal / verdict logic, generic over a `ReceiptVerifier` backend.
- CLI (`crates/verify-cli`) with the `check-journal` subcommand.
- Conformance suite reading `vectors/`: positive vectors must `PASS`; adversarial vectors (lookahead-contaminated, strategy-revealed, policy-mismatch) must be rejected.
- EVM verifier contract stub (`contracts/`) and TypeScript binding surface (`bindings/ts/`).
- CI: fmt · clippy (`-D warnings`) · build · test.
- Project docs: README, CONTRIBUTING, SECURITY, CODE_OF_CONDUCT.

[Unreleased]: https://github.com/muaviamohammed/zk-strategy-verify/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/muaviamohammed/zk-strategy-verify/releases/tag/v0.1.0
