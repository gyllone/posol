//SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.0;

import "../library/Bn254.sol";
import "../library/Domain.sol";
import "../library/KZGChecker.sol";

contract KZGMocker {
    function check() public view {
        Bn254.Fr memory point = Domain.element(10000);
        Bn254.Fr memory eval = Bn254.Fr(0x04ebbf9dc85e18e0eb200e65f46a55fc4cf95020adbde3f754e415c9e44fb7a0);
        Bn254.G1Point memory commitment = Bn254.G1Point(
            0x08619b0c2bc95ba3b5e6f720d4f5bcfceb62d455173d554ab7cefaaa4b42f09f,
            0x13fc65dd4e5c6b90d42b8741fe2821e82c0bd8a6d5a43d27a5221d9313f27d48
        );
        Bn254.G1Point memory opening = Bn254.G1Point(
            0x0359b99c601ddc4ef26271cede4449172f6b572a59d59cbf4944cb359de079c8,
            0x28f1013b27abaa40fb0fb3d1e5162809c2b23ad007430db6bb021f1702dfea04
        );

        bool result = KZGChecker.check(point, eval, opening, commitment);
        require(result, "KZG check failed");
    }

    function batchCheck(
        Bn254.Fr memory challenge,
        Bn254.Fr[] memory points,
        Bn254.Fr[] memory evals,
        Bn254.G1Point[] memory openings,
        Bn254.G1Point[] memory commitments
    ) public view {
        bool result = KZGChecker.batchCheck(challenge, points, evals, openings, commitments);
        require(result, "KZG batch check failed");
    }
}
