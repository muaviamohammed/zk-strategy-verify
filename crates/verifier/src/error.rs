use thiserror::Error;

/// Errors a verifier may return. An `Err` is a hard rejection: the receipt is
/// invalid or the journal does not conform to the declared policy. (SPEC.md §4)
#[derive(Debug, Error)]
pub enum VerifyError {
    #[error("receipt verification failed: {0}")]
    Receipt(String),

    #[error("unknown guest image id: {0}")]
    UnknownImage(String),

    #[error("journal decode error: {0}")]
    JournalDecode(String),

    #[error("gate policy mismatch: expected {expected}, found {found}")]
    PolicyMismatch { expected: String, found: String },

    #[error("dataset commitment (data_root) mismatch")]
    DatasetMismatch,

    #[error("dataset canonicalization mismatch")]
    CanonicalizationMismatch,

    #[error("strategy_hidden is false: a conforming proof must not reveal the strategy")]
    StrategyRevealed,

    #[error("receipt verification not yet implemented (enable the `risc0` feature, milestone M2)")]
    Unimplemented,
}
