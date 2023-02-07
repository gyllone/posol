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
    
    event ProofVerified(bool);

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
        emit ProofVerified(result);
    }
}