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
}

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
            digest: "0x1".into(),
        }
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
}
