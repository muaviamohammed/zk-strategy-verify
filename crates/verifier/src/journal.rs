use serde::{Deserialize, Serialize};

use crate::error::VerifyError;

/// The public values committed in a proof journal. (SPEC.md §2)
///
/// These are the only things a verifier sees. None of them reveal the strategy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Journal {
    /// Identifier of the guest program that produced the proof.
    pub image_id: String,
    /// Identifier/hash of the policy evaluated in-circuit.
    pub gate_policy: String,
    /// Commitment to the canonical dataset used.
    pub data_root: String,
    /// Commitment to the canonicalization rules (timezone, adjustments, point-in-time).
    pub dataset_canonicalization: String,
    /// In-circuit pass/fail over all declared conditions.
    pub verdict_pass: bool,
    /// Assertion that no strategy parameters are revealed in the journal.
    pub strategy_hidden: bool,
    /// Overall commitment binding the journal.
    pub digest: String,
}

impl Journal {
    /// Decode a journal from its canonical JSON byte encoding.
    ///
    /// M1 uses canonical JSON for clarity. M2 adds the compact binary encoding
    /// committed by the guest; the verifier is agnostic to the wire format so
    /// long as it decodes to this schema.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, VerifyError> {
        serde_json::from_slice(bytes).map_err(|e| VerifyError::JournalDecode(e.to_string()))
    }
}
