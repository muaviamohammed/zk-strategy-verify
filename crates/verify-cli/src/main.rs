//! CLI for the strategy-integrity verifier.
//!
//! Subcommand `check-journal` checks an already-verified journal against a gate
//! policy (the SPEC.md §2–§3 policy checks). It does NOT cryptographically verify
//! the receipt — that is the `risc0` backend, wired in milestone M2. Use it to
//! explore policy conformance against the test vectors.
//!
//!   zk-strategy-verify check-journal <policy.json> <journal.json>

use std::process::ExitCode;

use zk_strategy_verify::{GatePolicy, Journal};

fn usage() -> ExitCode {
    eprintln!(
        "usage:\n  zk-strategy-verify check-journal <policy.json> <journal.json>\n\n\
         Checks a journal against a gate policy (SPEC.md §2-§3).\n\
         Full receipt verification requires the `risc0` feature (milestone M2)."
    );
    ExitCode::from(2)
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("check-journal") if args.len() == 4 => run_check(&args[2], &args[3]),
        _ => usage(),
    }
}

fn run_check(policy_path: &str, journal_path: &str) -> ExitCode {
    let policy: GatePolicy = match read_json(policy_path) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("error reading policy: {e}");
            return ExitCode::from(2);
        }
    };
    let journal: Journal = match read_json(journal_path) {
        Ok(j) => j,
        Err(e) => {
            eprintln!("error reading journal: {e}");
            return ExitCode::from(2);
        }
    };

    match policy.evaluate(&journal) {
        Ok(report) if report.is_pass() => {
            println!(
                "PASS  gate_policy={} digest={}",
                report.gate_policy_id, report.digest
            );
            ExitCode::SUCCESS
        }
        Ok(report) => {
            println!(
                "FAIL  gate_policy={} digest={}",
                report.gate_policy_id, report.digest
            );
            ExitCode::FAILURE
        }
        Err(e) => {
            println!("REJECT  {e}");
            ExitCode::FAILURE
        }
    }
}

fn read_json<T: serde::de::DeserializeOwned>(path: &str) -> Result<T, String> {
    let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
    serde_json::from_slice(&bytes).map_err(|e| e.to_string())
}
