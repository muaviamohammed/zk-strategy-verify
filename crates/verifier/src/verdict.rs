use serde::{Deserialize, Serialize};

use crate::policy::Condition;

/// The pass/fail outcome of verification. (SPEC.md §2)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Verdict {
    Pass,
    Fail,
}

/// The typed result returned to a caller.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationReport {
    pub verdict: Verdict,
    pub gate_policy_id: String,
    pub conditions: Vec<Condition>,
    pub digest: String,
}

impl VerificationReport {
    pub fn is_pass(&self) -> bool {
        self.verdict == Verdict::Pass
    }
}
