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
    
    function testLastChallenge(
        BalanceSumVerifier.Proof memory proof,
        Bn254.Fr memory m
    ) public pure returns (uint256) {
        BalanceSumVerifier.Challenges memory challenges = proof.generateChallenges(m);
        return challenges.etas[0].value;
    }
}