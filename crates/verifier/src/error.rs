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

    #[error("unsupported credential format version {found} (this verifier implements up to {max_supported})")]
    UnsupportedFormatVersion { found: u16, max_supported: u16 },

    #[error("policy requires intrabar risk marking but the journal is close-marked (intrabar_marked=false)")]
    IntrabarRiskRequired,

    #[error("policy requires exposure disclosure but the journal carries no exposure card")]
    ExposureCardRequired,

    #[error(
        "exposure card is inconsistent (bar accounting, band ordering, or leverage rail violated)"
    )]
    ExposureCardInconsistent,

    #[error("policy requires the pinned regime panel but the journal carries none")]
    RegimePanelRequired,

    #[error("regime policy '{found}' is not the pinned policy — bucket-shopping is rejected")]
    RegimePolicyNotPinned { found: String },

    #[error("regime panel is inconsistent (bucket count or bar accounting violated)")]
    RegimePanelInconsistent,

    #[error("committed costs below the protocol floor (fee {fee_bps} bps / slippage {slippage_bps} bps; floor 7+3) — understated costs are a forge vector")]
    CostFloorViolated { fee_bps: i64, slippage_bps: i64 },

    #[error("receipt verification not yet implemented (enable the `risc0` feature, milestone M2)")]
    Unimplemented,
}
