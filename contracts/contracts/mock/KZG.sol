//SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.0;

import "../library/Bn254.sol";
import "../library/KZGChecker.sol";

contract KZGMocker {
    function check(
        Bn254.Fr memory point,
        Bn254.Fr memory eval,
        Bn254.G1Point memory opening,
        Bn254.G1Point memory commitment
    ) public view {
        Bn254.Fr memory point = Bn254.Fr(0x1);
        Bn254.Fr memory eval = Bn254.Fr(0x2);


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
