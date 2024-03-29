// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.0;

import "./Domain.sol";
import "./KZGChecker.sol";
import "../library/Bn254.sol";
import "../library/TranscriptProtocol.sol";
import "../library/BalanceSumVerifier.sol";

contract TestBalanceSumVerifier {
    using Bn254 for Bn254.Fr;
    using Bn254 for Bn254.G1Point;
    using TranscriptProtocol for TranscriptProtocol.Transcript;
    using BalanceSumVerifier for BalanceSumVerifier.Proof;

    struct PackedProof {
        BalanceSumVerifier.Proof proof;
        Bn254.Fr balanceSum;
    }

    struct CommittedData {
        uint256 timestamp;
        Bn254.G1Point tagCommit;
        Bn254.G1Point aggBalanceCommit;
        Bn254.Fr aggBalanceSum;
    }

    mapping(bytes32 => CommittedData[]) private committedDatas;

    function testLastChallenge(
        BalanceSumVerifier.Proof memory proof,
        Bn254.Fr memory m
    ) public pure returns (uint256) {
        BalanceSumVerifier.Challenges memory challenges = proof.generateChallenges(m);
        return challenges.etas[0].value;
    }

    function testEvaluation1(
        BalanceSumVerifier.Proof memory proof,
        Bn254.Fr memory m
    ) public view returns (uint256) {
        // Generate challenges via Fiat-Shamir algorithm
        BalanceSumVerifier.Challenges memory challenges = proof.generateChallenges(m);
        // Compute vanishing polynomial evaluation
        Bn254.Fr memory zh = BalanceSumVerifier.evaluateVanishingPoly(challenges.z);
        // Compute first Lagrange polynomial evaluation
        Bn254.Fr memory firstLagEval = BalanceSumVerifier.evaluateFirstLagrangePoly(challenges.z, zh);
        // Compute last Lagrange polynomial evaluation
        Bn254.Fr memory lastLagEval = BalanceSumVerifier.evaluateLastLagrangePoly(challenges.z, zh);

        Bn254.Fr memory eval1 = proof.computeEvaluation1(challenges, firstLagEval, lastLagEval, m);
        return eval1.value;
    }

    function testCommitment1(
        BalanceSumVerifier.Proof memory proof,
        Bn254.Fr memory m
    ) public view returns (uint256, uint256) {
        // Generate challenges via Fiat-Shamir algorithm
        BalanceSumVerifier.Challenges memory challenges = proof.generateChallenges(m);
        // Compute vanishing polynomial evaluation
        Bn254.Fr memory zh = BalanceSumVerifier.evaluateVanishingPoly(challenges.z);
        // Compute first Lagrange polynomial evaluation
        Bn254.Fr memory firstLagEval = BalanceSumVerifier.evaluateFirstLagrangePoly(challenges.z, zh);
        // Compute last Lagrange polynomial evaluation
        Bn254.Fr memory lastLagEval = BalanceSumVerifier.evaluateLastLagrangePoly(challenges.z, zh);

        Bn254.G1Point memory commitment1 = proof.linearisationCommitments1(
            challenges,
            zh,
            firstLagEval,
            lastLagEval
        );
        return (commitment1.x, commitment1.y);
    }

    function testVerifyProof(
        BalanceSumVerifier.Proof memory proof,
        Bn254.Fr memory balanceSum
    ) public {
        bool result = proof.verify(balanceSum);
        require(result, "failed to verify proof");
    }

    function testForCexProver(
        PackedProof[] memory proofs
    ) public {
        Bn254.G1Point memory aggBalanceCommit = Bn254.G1Point(0, 0);
        Bn254.Fr memory aggBalanceSum = Bn254.Fr(0);
        Bn254.Fr memory multiplier = Bn254.Fr(1);
        for (uint i = 0; i < proofs.length; i++) {
            PackedProof memory proof = proofs[i];
            // verify each balance sum proof
            bool result = proof.proof.verify(proof.balanceSum);
            require(result, "Failed verify balance sum proof");

            // aggregate balance sum
            proof.balanceSum.mulAssign(multiplier);
            aggBalanceSum.addAssign(proof.balanceSum);
            // aggregate balance commitment
            proof.proof.bCommit.pointMulAssign(multiplier);
            aggBalanceCommit.pointAddAssign(proof.proof.bCommit);
            // update multiplier
            multiplier.mulAssign(Bn254.Fr(Domain.SIZE));
        }
    }
}