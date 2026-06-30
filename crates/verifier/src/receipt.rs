use crate::error::VerifyError;

/// Abstraction over the cryptographic receipt verifier.
///
/// The library is generic over this trait so the policy/journal logic can be
/// tested deterministically without a zkVM, while the real RISC Zero backend is
/// wired in at milestone M2 behind the `risc0` feature.
pub trait ReceiptVerifier {
    /// Cryptographically verify `receipt_bytes` against one of `allowed_image_ids`,
    /// returning the committed journal bytes on success.
    fn verify_receipt(
        &self,
        allowed_image_ids: &[String],
        receipt_bytes: &[u8],
    ) -> Result<Vec<u8>, VerifyError>;
}

/// RISC Zero backend. Wired in milestone M2; requires the `risc0` feature.
///
/// M2 implementation: call `risc0_zkvm::Receipt::verify(image_id)` for each
/// allowed image id, and on success return `receipt.journal.bytes`.
#[cfg(feature = "risc0")]
pub struct Risc0Verifier;

#[cfg(feature = "risc0")]
impl ReceiptVerifier for Risc0Verifier {
    fn verify_receipt(
        &self,
        _allowed_image_ids: &[String],
        _receipt_bytes: &[u8],
    ) -> Result<Vec<u8>, VerifyError> {
        Err(VerifyError::Unimplemented)
    }
}
