use serde::{Deserialize, Serialize};

use crate::error::VerifyError;

/// (v3, disclosure tier) In-circuit exposure accounting. (SPEC.md §3, condition 7)
///
/// Derived inside the circuit from the SAME position × leverage series that
/// produced the P&L — it cannot describe a different book than the one that
/// earned the returns. All ratios are fixed-point ×100 (150 = 1.50×).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExposureCard {
    /// Average / minimum / maximum net exposure over the window (signed, ×100).
    pub net_avg_x100: i32,
    pub net_min_x100: i32,
    pub net_max_x100: i32,
    /// Average / maximum gross exposure (unsigned, ×100).
    pub gross_avg_x100: u32,
    pub gross_max_x100: u32,
    /// The leverage rail the strategy declared; gross may never exceed it.
    pub leverage_cap_x100: u32,
    /// Number of instruments in the book.
    pub instruments: u32,
    /// Bar accounting: every bar in the window is long, short, or flat.
    pub bars_long: u64,
    pub bars_short: u64,
    pub bars_flat: u64,
}

impl ExposureCard {
    /// Structural consistency (SPEC.md §3.7): bar accounting must cover the
    /// committed window exactly, bands must be ordered, gross must respect the
    /// leverage rail, and net can never exceed gross.
    pub fn is_consistent(&self, window_bars: u64) -> bool {
        self.bars_long + self.bars_short + self.bars_flat == window_bars
            && self.net_min_x100 <= self.net_avg_x100
            && self.net_avg_x100 <= self.net_max_x100
            && self.gross_avg_x100 <= self.gross_max_x100
            && self.gross_max_x100 <= self.leverage_cap_x100
            && self.net_max_x100.unsigned_abs() <= self.gross_max_x100
            && self.net_min_x100.unsigned_abs() <= self.gross_max_x100
            && self.instruments >= 1
    }
}

/// (v4, disclosure tier) One bucket of regime-conditional performance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegimeBucket {
    /// Mean per-bar return in this regime (bps ×100 fixed-point).
    pub mean_return_bps_x100: i64,
    /// Bars labelled into this regime (strictly causal labels).
    pub bars: u64,
}

/// (v4, disclosure tier) Regime-conditional performance under a PINNED policy.
/// (SPEC.md §3, condition 8)
///
/// The regime policy (bucket definitions) is a protocol constant — the prover
/// cannot shop for the bucketing that flatters the strategy. A full verifier
/// additionally re-derives the bucket bar-counts from its own copy of the
/// public dataset; this reference implementation checks structure + pinning.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegimePanel {
    /// Identifier of the regime policy used; MUST equal the pinned constant
    /// ([`crate::policy::PINNED_REGIME_POLICY_ID`]).
    pub policy_id: String,
    /// Exactly 6 buckets under the pinned policy (3 vol terciles × 2 trend states).
    pub buckets: Vec<RegimeBucket>,
}

impl RegimePanel {
    /// Structural consistency (SPEC.md §3.8): exactly 6 buckets under the
    /// pinned policy, at least one labelled bar, and total labelled bars can
    /// never exceed the committed window (causal labels need warm-up, so the
    /// sum is ≤ the window, not equal to it).
    pub fn is_consistent(&self, window_bars: u64) -> bool {
        let total: u64 = self.buckets.iter().map(|b| b.bars).sum();
        self.buckets.len() == 6 && total > 0 && total <= window_bars
    }
}

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
    /// Credential format version committed in-circuit (SPEC.md §5). A verifier
    /// MUST reject a version it does not implement rather than misinterpret the
    /// journal. `1` = close-marked risk; `2` adds OHLC/intrabar marking and the
    /// bound annualization basis. Absent in a v1 journal → deserializes to `1`.
    #[serde(default = "default_format_version")]
    pub format_version: u16,
    /// True iff drawdown / worst-bar were marked against real intrabar extremes
    /// (OHLC data). False = close-marked risk (no intrabar information). The
    /// verifier surfaces this so close-marked risk is never mistaken for
    /// intrabar-marked risk. Absent in a v1 journal → `false`.
    #[serde(default)]
    pub intrabar_marked: bool,
    /// (v3+) Number of bars in the committed evaluation window. Required by the
    /// disclosure-tier consistency checks (bar accounting). Absent → `0`
    /// (pre-v3 journal; disclosure conditions cannot be required against it).
    #[serde(default)]
    pub window_bars: u64,
    /// (v3, disclosure tier) In-circuit exposure accounting. `None` for
    /// output-minimized (PASS/FAIL-only) credentials — disclosure is opt-in.
    #[serde(default)]
    pub exposure_card: Option<ExposureCard>,
    /// (v4, disclosure tier) Regime-conditional performance under the PINNED
    /// regime policy. `None` for output-minimized credentials.
    #[serde(default)]
    pub regime_panel: Option<RegimePanel>,
    /// Committed fee (bps per position change) applied inside the proof.
    /// Absent in a pre-cost-disclosure journal → defaults to the protocol
    /// floor (legacy journals were minted AT the floor by construction).
    #[serde(default = "default_fee_bps")]
    pub fee_bps: i64,
    /// Committed slippage (bps per position change). Same legacy default.
    #[serde(default = "default_slippage_bps")]
    pub slippage_bps: i64,
    /// Overall commitment binding the journal.
    pub digest: String,
}

/// Protocol cost floor (SPEC.md §3.9). Legacy journals default to it.
fn default_fee_bps() -> i64 {
    crate::policy::COST_FLOOR_FEE_BPS
}
fn default_slippage_bps() -> i64 {
    crate::policy::COST_FLOOR_SLIPPAGE_BPS
}

/// v1 journals predate the `format_version` field; a missing value means v1.
fn default_format_version() -> u16 {
    1
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
