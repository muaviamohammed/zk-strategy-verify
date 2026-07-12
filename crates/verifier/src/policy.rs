use serde::{Deserialize, Serialize};

use crate::error::VerifyError;
use crate::journal::Journal;
use crate::verdict::{Verdict, VerificationReport};

/// An integrity condition a gate policy may require. (SPEC.md §3)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Condition {
    /// Returns are net of declared costs/fees/slippage.
    AfterCosts,
    /// Signals at time `t` use only information available at `t`.
    NoLookahead,
    /// Performance holds on a held-out segment not used for selection.
    OutOfSample,
    /// Performance holds under rolling re-fit / re-test.
    WalkForward,
    /// (v2) Drawdown and worst-bar are marked against the ADVERSE INTRABAR
    /// extreme at the position's leverage — a levered wick that nearly
    /// liquidates but recovers by the close is not hidden. Requires OHLC data
    /// (`journal.intrabar_marked == true`).
    IntrabarRisk,
    /// (v2) The annualization basis (periods-per-year used to scale Sharpe/CAGR)
    /// is bound in-circuit to the committed bar/print timestamps, so it cannot
    /// be inflated to forge a passing Sharpe. Bundles both the price-path
    /// binding and the timestamped-funding carry binding.
    AnnualizationBound,
    /// (v3, disclosure tier) The journal carries an in-circuit exposure card —
    /// net/gross bands, leverage rail, instrument count, and long/short/flat
    /// bar accounting — derived from the SAME position series as the P&L.
    /// Answers the allocator's "what am I actually exposed to?" without
    /// revealing the strategy.
    ExposureDisclosed,
    /// (v4, disclosure tier) The journal carries regime-conditional performance
    /// under a PINNED regime policy (protocol constant): the prover cannot
    /// choose the bucketing that flatters the strategy, and labels are
    /// strictly causal. Answers "what should I expect from this strategy in
    /// the current regime?".
    RegimePinned,
}

/// The pinned regime policy (SPEC.md §3.8): 30-bar trailing-vol terciles ×
/// 100-bar trend state, labels strictly causal. A journal claiming any other
/// regime policy is rejected — bucket-shopping is a forgery class, not a choice.
pub const PINNED_REGIME_POLICY_ID: &str = "vol30-trend100";

/// Highest credential format version this verifier implements. A journal
/// declaring a newer format is REJECTED (not misread) — see SPEC.md §5.
pub const MAX_SUPPORTED_FORMAT_VERSION: u16 = 4;

/// A versioned gate policy. The verifier checks a decoded [`Journal`] against this.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatePolicy {
    pub spec_version: String,
    /// Identifier/hash of the policy as evaluated in-circuit; must equal `journal.gate_policy`.
    pub gate_policy_id: String,
    /// Guest image ids whose receipts are accepted.
    pub allowed_image_ids: Vec<String>,
    /// Required dataset commitment; must equal `journal.data_root`.
    pub data_root: String,
    /// Required canonicalization commitment; must equal `journal.dataset_canonicalization`.
    pub dataset_canonicalization: String,
    /// Conditions this policy requires (informational to the caller; the
    /// authoritative pass/fail over them is committed in `journal.verdict_pass`).
    pub required_conditions: Vec<Condition>,
}

impl GatePolicy {
    /// Evaluate a decoded journal against this policy. (SPEC.md §2–§3)
    ///
    /// Returns `Ok(Fail)` when the in-circuit verdict was negative, and `Err`
    /// when the journal does not conform to the policy at all (mismatched policy,
    /// dataset, canonicalization, or a revealed strategy).
    pub fn evaluate(&self, journal: &Journal) -> Result<VerificationReport, VerifyError> {
        if journal.gate_policy != self.gate_policy_id {
            return Err(VerifyError::PolicyMismatch {
                expected: self.gate_policy_id.clone(),
                found: journal.gate_policy.clone(),
            });
        }
        if journal.data_root != self.data_root {
            return Err(VerifyError::DatasetMismatch);
        }
        if journal.dataset_canonicalization != self.dataset_canonicalization {
            return Err(VerifyError::CanonicalizationMismatch);
        }
        if !journal.strategy_hidden {
            return Err(VerifyError::StrategyRevealed);
        }
        // (v2) Reject a format the verifier does not implement rather than
        // silently misread the journal.
        if journal.format_version > MAX_SUPPORTED_FORMAT_VERSION {
            return Err(VerifyError::UnsupportedFormatVersion {
                found: journal.format_version,
                max_supported: MAX_SUPPORTED_FORMAT_VERSION,
            });
        }
        // (v2) If the policy REQUIRES intrabar risk marking, the journal must
        // attest it — a close-marked journal cannot satisfy an intrabar policy.
        if self.required_conditions.contains(&Condition::IntrabarRisk) && !journal.intrabar_marked {
            return Err(VerifyError::IntrabarRiskRequired);
        }
        // (v3, disclosure tier) A policy that requires the exposure card must
        // find one, over a known window, and it must be internally consistent.
        if self
            .required_conditions
            .contains(&Condition::ExposureDisclosed)
        {
            match &journal.exposure_card {
                None => return Err(VerifyError::ExposureCardRequired),
                Some(card) => {
                    if journal.window_bars == 0 || !card.is_consistent(journal.window_bars) {
                        return Err(VerifyError::ExposureCardInconsistent);
                    }
                }
            }
        }
        // (v4, disclosure tier) A policy that requires the regime panel must
        // find one under the PINNED policy — any other bucketing is rejected.
        if self.required_conditions.contains(&Condition::RegimePinned) {
            match &journal.regime_panel {
                None => return Err(VerifyError::RegimePanelRequired),
                Some(panel) => {
                    if panel.policy_id != PINNED_REGIME_POLICY_ID {
                        return Err(VerifyError::RegimePolicyNotPinned {
                            found: panel.policy_id.clone(),
                        });
                    }
                    if journal.window_bars == 0 || !panel.is_consistent(journal.window_bars) {
                        return Err(VerifyError::RegimePanelInconsistent);
                    }
                }
            }
        }

        let verdict = if journal.verdict_pass {
            Verdict::Pass
        } else {
            Verdict::Fail
        };

        Ok(VerificationReport {
            verdict,
            gate_policy_id: self.gate_policy_id.clone(),
            conditions: self.required_conditions.clone(),
            digest: journal.digest.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::VerifyError;
    use crate::journal::{ExposureCard, RegimeBucket, RegimePanel};

    fn policy() -> GatePolicy {
        GatePolicy {
            spec_version: "0.1".into(),
            gate_policy_id: "gp".into(),
            allowed_image_ids: vec!["img".into()],
            data_root: "dr".into(),
            dataset_canonicalization: "canon".into(),
            required_conditions: vec![Condition::AfterCosts, Condition::NoLookahead],
        }
    }

    fn journal() -> Journal {
        Journal {
            image_id: "img".into(),
            gate_policy: "gp".into(),
            data_root: "dr".into(),
            dataset_canonicalization: "canon".into(),
            verdict_pass: true,
            strategy_hidden: true,
            format_version: 2,
            intrabar_marked: true,
            window_bars: 0,
            exposure_card: None,
            regime_panel: None,
            digest: "0x1".into(),
        }
    }

    fn card() -> ExposureCard {
        ExposureCard {
            net_avg_x100: 55,
            net_min_x100: 0,
            net_max_x100: 100,
            gross_avg_x100: 55,
            gross_max_x100: 100,
            leverage_cap_x100: 100,
            instruments: 1,
            bars_long: 606,
            bars_short: 0,
            bars_flat: 490,
        }
    }

    fn panel() -> RegimePanel {
        RegimePanel {
            policy_id: PINNED_REGIME_POLICY_ID.into(),
            buckets: (0..6)
                .map(|i| RegimeBucket {
                    mean_return_bps_x100: i * 100,
                    bars: 160,
                })
                .collect(),
        }
    }

    fn disclosing_journal() -> Journal {
        let mut j = journal();
        j.format_version = 4;
        j.window_bars = 1096;
        j.exposure_card = Some(card());
        j.regime_panel = Some(panel());
        j
    }

    fn disclosing_policy() -> GatePolicy {
        let mut p = policy();
        p.required_conditions
            .extend([Condition::ExposureDisclosed, Condition::RegimePinned]);
        p
    }

    #[test]
    fn conforming_journal_passes() {
        let report = policy().evaluate(&journal()).unwrap();
        assert!(report.is_pass());
        assert_eq!(report.gate_policy_id, "gp");
    }

    #[test]
    fn negative_in_circuit_verdict_fails_not_errors() {
        let mut j = journal();
        j.verdict_pass = false;
        let report = policy().evaluate(&j).unwrap();
        assert!(!report.is_pass());
    }

    #[test]
    fn revealed_strategy_is_rejected() {
        let mut j = journal();
        j.strategy_hidden = false;
        assert!(matches!(
            policy().evaluate(&j),
            Err(VerifyError::StrategyRevealed)
        ));
    }

    #[test]
    fn policy_mismatch_is_rejected() {
        let mut j = journal();
        j.gate_policy = "other".into();
        assert!(matches!(
            policy().evaluate(&j),
            Err(VerifyError::PolicyMismatch { .. })
        ));
    }

    #[test]
    fn dataset_mismatch_is_rejected() {
        let mut j = journal();
        j.data_root = "other".into();
        assert!(matches!(
            policy().evaluate(&j),
            Err(VerifyError::DatasetMismatch)
        ));
    }

    #[test]
    fn canonicalization_mismatch_is_rejected() {
        let mut j = journal();
        j.dataset_canonicalization = "other".into();
        assert!(matches!(
            policy().evaluate(&j),
            Err(VerifyError::CanonicalizationMismatch)
        ));
    }

    #[test]
    fn future_format_version_is_rejected() {
        let mut j = journal();
        j.format_version = MAX_SUPPORTED_FORMAT_VERSION + 1;
        assert!(matches!(
            policy().evaluate(&j),
            Err(VerifyError::UnsupportedFormatVersion { .. })
        ));
    }

    #[test]
    fn intrabar_policy_rejects_close_marked_journal() {
        let mut p = policy();
        p.required_conditions.push(Condition::IntrabarRisk);
        let mut j = journal();
        j.intrabar_marked = false;
        assert!(matches!(
            p.evaluate(&j),
            Err(VerifyError::IntrabarRiskRequired)
        ));
        // an intrabar-marked journal satisfies the same policy
        j.intrabar_marked = true;
        assert!(p.evaluate(&j).unwrap().is_pass());
    }

    #[test]
    fn v1_journal_defaults_are_backward_compatible() {
        // a journal encoded before the v2 fields deserializes to v1 / close-marked
        let raw = r#"{"image_id":"img","gate_policy":"gp","data_root":"dr",
            "dataset_canonicalization":"canon","verdict_pass":true,
            "strategy_hidden":true,"digest":"0x1"}"#;
        let j = Journal::from_bytes(raw.as_bytes()).unwrap();
        assert_eq!(j.format_version, 1);
        assert!(!j.intrabar_marked);
        assert!(policy().evaluate(&j).unwrap().is_pass());
    }

    #[test]
    fn disclosing_journal_passes_disclosure_policy() {
        let report = disclosing_policy().evaluate(&disclosing_journal()).unwrap();
        assert!(report.is_pass());
    }

    #[test]
    fn disclosure_policy_rejects_missing_exposure_card() {
        let mut j = disclosing_journal();
        j.exposure_card = None;
        assert!(matches!(
            disclosing_policy().evaluate(&j),
            Err(VerifyError::ExposureCardRequired)
        ));
    }

    #[test]
    fn inconsistent_exposure_card_is_rejected() {
        let mut j = disclosing_journal();
        // bar accounting no longer covers the window
        j.exposure_card.as_mut().unwrap().bars_flat = 0;
        assert!(matches!(
            disclosing_policy().evaluate(&j),
            Err(VerifyError::ExposureCardInconsistent)
        ));
    }

    #[test]
    fn gross_exceeding_leverage_rail_is_rejected() {
        let mut j = disclosing_journal();
        j.exposure_card.as_mut().unwrap().gross_max_x100 = 200; // rail is 100
        assert!(matches!(
            disclosing_policy().evaluate(&j),
            Err(VerifyError::ExposureCardInconsistent)
        ));
    }

    #[test]
    fn shopped_regime_policy_is_rejected() {
        let mut j = disclosing_journal();
        j.regime_panel.as_mut().unwrap().policy_id = "vol10-trend20-custom".into();
        assert!(matches!(
            disclosing_policy().evaluate(&j),
            Err(VerifyError::RegimePolicyNotPinned { .. })
        ));
    }

    #[test]
    fn regime_panel_overclaiming_bars_is_rejected() {
        let mut j = disclosing_journal();
        // 6 × 300 = 1800 labelled bars > 1096-bar window
        for b in &mut j.regime_panel.as_mut().unwrap().buckets {
            b.bars = 300;
        }
        assert!(matches!(
            disclosing_policy().evaluate(&j),
            Err(VerifyError::RegimePanelInconsistent)
        ));
    }

    #[test]
    fn minimal_journal_still_passes_base_policy() {
        // disclosure is OPT-IN: a PASS/FAIL-only journal under a base policy
        // (no disclosure conditions) remains fully verifiable
        let report = policy().evaluate(&journal()).unwrap();
        assert!(report.is_pass());
    }
}
