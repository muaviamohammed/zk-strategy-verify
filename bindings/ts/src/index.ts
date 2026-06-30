// TypeScript bindings for the strategy-integrity verifier. (milestone M2)
//
// These bindings will wrap the Rust `zk-strategy-verify` crate compiled to wasm
// (wasm-bindgen), exposing the same SPEC.md §2-§3 verification surface to JS/TS
// consumers (allocator dashboards, off-chain checks). Types are declared now so
// downstream integrators can build against a stable interface.

export type Verdict = "PASS" | "FAIL";

export type Condition =
  | "after_costs"
  | "no_lookahead"
  | "out_of_sample"
  | "walk_forward";

export interface GatePolicy {
  spec_version: string;
  gate_policy_id: string;
  allowed_image_ids: string[];
  data_root: string;
  dataset_canonicalization: string;
  required_conditions: Condition[];
}

export interface VerificationReport {
  verdict: Verdict;
  gate_policy_id: string;
  conditions: Condition[];
  digest: string;
}

/** Verify a receipt against a gate policy. Implemented in M2 (wasm). */
export async function verify(
  _policy: GatePolicy,
  _receiptBytes: Uint8Array,
): Promise<VerificationReport> {
  throw new Error("not yet implemented: wasm verifier lands in milestone M2");
}
