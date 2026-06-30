// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.24;

/// @title StrategyIntegrityVerifier
/// @notice EVM verifier for integrity-proofs of systematic strategies. (SPEC.md §2)
/// @dev Milestone M3. Thin wrapper over the on-chain proof verifier (a RISC Zero
///      Groth16-wrapped receipt) that additionally enforces the gate-policy
///      commitments carried in the proof journal. A consuming protocol can require
///      `verify(...) == PASS` as a condition of capital flow. The contract never
///      sees the strategy — only the public journal commitments.
interface IRiscZeroVerifier {
    /// @notice Verify a (wrapped) receipt for guest `imageId` committing `journalDigest`.
    function verify(bytes calldata seal, bytes32 imageId, bytes32 journalDigest) external view;
}

contract StrategyIntegrityVerifier {
    /// @notice Decoded public commitments from the proof journal. (SPEC.md §2)
    struct Journal {
        bytes32 imageId;
        bytes32 gatePolicy;
        bytes32 dataRoot;
        bytes32 datasetCanonicalization;
        bool verdictPass;
        bool strategyHidden;
        bytes32 digest;
    }

    IRiscZeroVerifier public immutable RISC_ZERO;

    constructor(IRiscZeroVerifier riscZero) {
        RISC_ZERO = riscZero;
    }

    /// @notice Verify a receipt and enforce a declared gate policy.
    /// @return pass True iff the receipt is valid AND the journal conforms to the policy.
    function verify(
        bytes calldata seal,
        Journal calldata journal,
        bytes32 expectedGatePolicy,
        bytes32 expectedDataRoot,
        bytes32 expectedCanonicalization,
        bytes32[] calldata allowedImageIds
    ) external view returns (bool pass) {
        // M3 TODO: compute journalDigest from the canonical journal encoding and
        // call RISC_ZERO.verify(seal, journal.imageId, journalDigest) (reverts on
        // invalid receipt). Below are the policy checks (SPEC.md §2-§3).

        if (!_imageAllowed(journal.imageId, allowedImageIds)) return false;
        if (journal.gatePolicy != expectedGatePolicy) return false;
        if (journal.dataRoot != expectedDataRoot) return false;
        if (journal.datasetCanonicalization != expectedCanonicalization) return false;
        if (!journal.strategyHidden) return false; // must not reveal the strategy

        return journal.verdictPass;
    }

    function _imageAllowed(bytes32 imageId, bytes32[] calldata allowed)
        internal
        pure
        returns (bool)
    {
        for (uint256 i = 0; i < allowed.length; i++) {
            if (allowed[i] == imageId) return true;
        }
        return false;
    }
}
