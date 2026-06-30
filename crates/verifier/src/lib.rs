//! Reference verifier for zero-knowledge integrity-proofs of systematic strategies.
//!
//! A verifier *checks* a proof; it does not generate one and learns nothing about
//! the strategy. See `SPEC.md` for the normative gate-policy specification.
//!
//! The crate is generic over a [`ReceiptVerifier`] so the policy/journal logic is
//! testable deterministically without a zkVM; the RISC Zero backend is wired in at
//! milestone M2 behind the `risc0` feature.

pub mod error;
pub mod journal;
pub mod policy;
pub mod receipt;
pub mod verdict;

pub use error::VerifyError;
pub use journal::Journal;
pub use policy::{Condition, GatePolicy};
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
