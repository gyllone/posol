// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.0;

import "./Bn254.sol";

library KZGChecker {
    using Bn254 for Bn254.Fr;
    using Bn254 for Bn254.G1Point;

    function pointG() internal pure returns (Bn254.G1Point memory) {
        return Bn254.G1Point(
            0x1ae3ae77e4bae16dc07d0f43622cc8835143ed3957df5b9b3dec20d1b3a1c546,
            0x2e8ff358e0d435b26342e24db5db2e1a1365de5bab56de591c1775c44edf73c1
        );
    }

    function pointH() internal pure returns (Bn254.G2Point memory) {
        return Bn254.G2Point(
            [
                0x2ee2102c718f1f9d8a59a1cae20c6253af391aafb53a3cfa73e6610a931a5302,
                0x0f0936e100281a45b0768dfb1463cd67287443927e59fffc538be08caa06c459
            ],
            [
                0x24a795d278f59445131295993962ee26056217efc1b3c1420dab196aacf686f0,
                0x043673183bfd89ef388ee5eb48de3ced289ec31b96650ddc1f795ee8c9a84407
            ]
        );
    }

    function pointBetaH() internal pure returns (Bn254.G2Point memory) {
        return Bn254.G2Point(
            [
                0x0205238886dab0bbd5326f37072d4e59033807777f1dd0ac69b6a576d4298df7,
                0x250509b6fd346fb4986ceb3e3c2ae1352b91a2899c44a9bff7347f9309d5b2bb
            ],
            [
                0x28d625b881fd1da41d022112823ad7e55d43259bb37cd1bf7e08838fb5c35a7d,
                0x0850c0444822f06b1f53da68c3ff950b1864b1982df33d23ea342ba16b713917
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