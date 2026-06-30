//! Conformance suite. (SPEC.md §4)
//!
//! Positive vectors in `vectors/pass/` MUST verify to PASS. Adversarial vectors
//! in `vectors/fail/` MUST be rejected — either an `Err`, or a non-PASS verdict.
//! A verifier that accepts any `fail/` vector is non-conforming.

use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use zk_strategy_verify::{verify, GatePolicy, Journal, ReceiptVerifier, VerifyError};

/// Deterministic stand-in for the cryptographic backend: it treats the receipt
/// bytes AS the committed journal bytes (i.e. assumes a valid receipt), so the
/// suite exercises the policy/journal conformance logic without a zkVM. The real
/// cryptographic step is the `risc0` backend (M2).
struct ReplayBackend;
impl ReceiptVerifier for ReplayBackend {
    fn verify_receipt(&self, _allowed: &[String], receipt: &[u8]) -> Result<Vec<u8>, VerifyError> {
        Ok(receipt.to_vec())
    }
}

#[derive(Deserialize)]
struct Vector {
    policy: GatePolicy,
    journal: Journal,
}

fn vectors_dir(kind: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../vectors")
        .join(kind)
}

fn load_vectors(kind: &str) -> Vec<(String, Vector)> {
    let dir = vectors_dir(kind);
    let mut out = Vec::new();
    for entry in fs::read_dir(&dir).unwrap_or_else(|_| panic!("missing vectors dir: {dir:?}")) {
        let path = entry.unwrap().path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let raw = fs::read(&path).unwrap();
        let v: Vector =
            serde_json::from_slice(&raw).unwrap_or_else(|e| panic!("bad vector {path:?}: {e}"));
        out.push((path.file_name().unwrap().to_string_lossy().into_owned(), v));
    }
    assert!(!out.is_empty(), "no vectors found in {dir:?}");
    out
}

#[test]
fn pass_vectors_verify_to_pass() {
    for (name, v) in load_vectors("pass") {
        let journal_bytes = serde_json::to_vec(&v.journal).unwrap();
        let report = verify(&ReplayBackend, &v.policy, &journal_bytes)
            .unwrap_or_else(|e| panic!("pass vector {name} errored: {e}"));
        assert!(
            report.is_pass(),
            "pass vector {name} did not verify to PASS"
        );
    }
}

#[test]
fn fail_vectors_are_rejected() {
    for (name, v) in load_vectors("fail") {
        let journal_bytes = serde_json::to_vec(&v.journal).unwrap();
        match verify(&ReplayBackend, &v.policy, &journal_bytes) {
            Err(_) => {} // rejected — conforming
            Ok(report) => assert!(
                !report.is_pass(),
                "fail vector {name} was accepted as PASS — non-conforming"
            ),
        }
    }
}
