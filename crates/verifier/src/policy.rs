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
