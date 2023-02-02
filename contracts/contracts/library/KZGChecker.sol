// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.0;

import "./Bn254.sol";

library KZGChecker {
    using Bn254 for Bn254.Fr;
    using Bn254 for Bn254.G1Point;

    function pointG() internal pure returns (Bn254.G1Point memory) {
        return Bn254.G1Point(
            0x29798bcb86bb7555185b140db83d0eba70144dcfe1c3e78266c6b4e6a9f2d860,
            0x030e94f3b741705ef60d16d366788ebf6739da0e1d086f9f781c239dea42b019
        );
    }

    function pointH() internal pure returns (Bn254.G2Point memory) {
        return Bn254.G2Point(
            [
                0x0fd29b3a93d4fff876bb6ae4747c28fa5ebce32fc47f0eb0aceb8b3d229354fd,
                0x29556e2e341ff77032548b0c2306352020e751f312af1722f0a88a8dd8805e62
            ],
            [
                0x1487fc314c94733ed9c7015ddc38d25c9636869029b6b4386b9f808e9584f151,
                0x175982e84d014441d058b20f790906cc4096a09e7ceaf4f9f9b9b8ce8229f924
            ]
        );
    }

    function pointBetaH() internal pure returns (Bn254.G2Point memory) {
        return Bn254.G2Point(
            [
                0x0fc5f9115b7fa93f02bc063540979ef5811e1bdbb28097166007e4285ef485b2,
                0x014900bf9af790006ba8b31a9e9f55097f7d57ab68a783718d1b309087a417ac
            ],
            [
                0x0b5249815f49d86953704b225156a08079ab1c7093984b815c01c156dd317e3e,
                0x22d8a9196bd765bae4df8148dcb9401d68cc85da586f6db30abafdcfcd3adbcf
            ]
        );
    }

    function check(
        Bn254.Fr memory point,
        Bn254.Fr memory eval,
        Bn254.G1Point memory opening,
        Bn254.G1Point memory commitment
    ) internal view returns (bool) {
        Bn254.G1Point memory g = pointG();
        Bn254.G2Point memory h = pointH();
        Bn254.G2Point memory betaH = pointBetaH();

        g.pointMulAssign(eval);
        g.pointSubAssign(commitment);
        g.pointSubAssign(opening.pointMul(point));

        return Bn254.pairingProd2(opening, betaH, g, h);
    }

    function batchCheck(
        Bn254.Fr memory challenge,
        Bn254.Fr[] memory points,
        Bn254.Fr[] memory evals,
        Bn254.G1Point[] memory openings,
        Bn254.G1Point[] memory commitments
    ) internal view returns (bool) {
        require(points.length == evals.length, "Unmatched array length");
        require(points.length == openings.length, "Unmatched array length");
        require(points.length == commitments.length, "Unmatched array length");
        
        Bn254.G1Point memory g = pointG();
        Bn254.G2Point memory h = pointH();
        Bn254.G2Point memory betaH = pointBetaH();

        Bn254.Fr memory u = Bn254.Fr(1);
        Bn254.Fr memory tmpFr = Bn254.Fr(0);
        Bn254.G1Point memory partA = Bn254.G1Point(0, 0);
        Bn254.G1Point memory partB = Bn254.G1Point(0, 0);
        Bn254.G1Point memory tmpG1 = Bn254.G1Point(0, 0);
        for (uint256 i = 0; i < points.length; i++) {
            tmpG1.copyFromG1(openings[i]);
            tmpG1.pointMulAssign(u);
            partA.pointAddAssign(tmpG1);

            tmpFr.copyFromFr(evals[i]);
            tmpFr.mulAssign(u);
            tmpG1.copyFromG1(g);
            tmpG1.pointMulAssign(tmpFr);
            partB.pointAddAssign(tmpG1);
            tmpG1.copyFromG1(commitments[i]);
            tmpG1.pointMulAssign(u);
            partB.pointSubAssign(tmpG1);
            tmpFr.copyFromFr(points[i]);
            tmpFr.mulAssign(u);
            tmpG1.copyFromG1(openings[i]);
            tmpG1.pointMulAssign(tmpFr);
            partB.pointSubAssign(tmpG1);

            u.mulAssign(challenge);
        }
        // Pairing check
        return Bn254.pairingProd2(partA, betaH, partB, h);
    }
}