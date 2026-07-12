//! Reference verifier for zero-knowledge integrity-proofs of systematic strategies.
//!
//! A verifier *checks* a proof; it does not generate one and learns nothing about
//! the strategy. See `SPEC.md` for the normative gate-policy specification.
//!
//! The crate is generic over a [`ReceiptVerifier`] so the policy/journal logic is
//! testable deterministically without a zkVM; the RISC Zero backend is wired in at
//! milestone M2 behind the `risc0` feature.
//!
//! # Example
//!
//! Check a decoded journal against a gate policy (the SPEC.md §2–§3 checks):
//!
//! ```
//! use zk_strategy_verify::{Condition, GatePolicy, Journal};
//!
//! let policy = GatePolicy {
//!     spec_version: "0.1".into(),
//!     gate_policy_id: "gp".into(),
//!     allowed_image_ids: vec!["img".into()],
//!     data_root: "dr".into(),
//!     dataset_canonicalization: "canon".into(),
//!     required_conditions: vec![Condition::AfterCosts, Condition::NoLookahead],
//! };
//!
//! let journal = Journal {
//!     image_id: "img".into(),
//!     gate_policy: "gp".into(),
//!     data_root: "dr".into(),
//!     dataset_canonicalization: "canon".into(),
//!     verdict_pass: true,
//!     strategy_hidden: true,
//!     format_version: 2,
//!     intrabar_marked: true,
//!     window_bars: 0,
//!     exposure_card: None,
//!     regime_panel: None,
//!     digest: "0x1".into(),
//! };
//!
//! let report = policy.evaluate(&journal).expect("conforms to policy");
//! assert!(report.is_pass());
//! ```

pub mod error;
pub mod journal;
pub mod policy;
pub mod receipt;
pub mod verdict;

pub use error::VerifyError;
pub use journal::{ExposureCard, Journal, RegimeBucket, RegimePanel};
pub use policy::{Condition, GatePolicy, PINNED_REGIME_POLICY_ID};
pub use receipt::ReceiptVerifier;
pub use verdict::{Verdict, VerificationReport};

/// Verify a receipt against a declared gate policy. (SPEC.md §2–§3)
///
/// 1. cryptographically verify the receipt against `policy.allowed_image_ids`,
///    recovering the committed journal bytes;
/// 2. decode the journal;
/// 3. check the journal's commitments satisfy the declared policy;
/// 4. require the in-circuit verdict to be PASS and `strategy_hidden` to hold.
pub fn verify<V: ReceiptVerifier>(
    backend: &V,
    policy: &GatePolicy,
    receipt_bytes: &[u8],
) -> Result<VerificationReport, VerifyError> {
    let journal_bytes = backend.verify_receipt(&policy.allowed_image_ids, receipt_bytes)?;
    let journal = Journal::from_bytes(&journal_bytes)?;
    policy.evaluate(&journal)
}
