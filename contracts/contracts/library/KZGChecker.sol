// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.0;

import "./Bn254.sol";

library KZGChecker {
    using Bn254 for Bn254.Fr;
    using Bn254 for Bn254.G1Point;

    function pointG() internal pure returns (Bn254.G1Point memory) {
        return Bn254.G1Point(
            0x16ec48c31f874dba2a3b3f74107bf12ad45c8353f403778a8e5d17df25e07cdf,
            0x20f2c5686fccbfad42ac39b7d2dfa5d241b2e423ddb6a5d05060bed389bfbc63
        );
    }

    function pointH() internal pure returns (Bn254.G2Point memory) {
        return Bn254.G2Point(
            [
                0x2e693da4ee4ba44597b16c46bd7868b0196ec9a5952aca0b0dfb5112fc7df1b4,
                0x070a5ebd1d20576e40620c6540f1d90e521b72c428638fd60a421a13755009b6
            ],
            [
                0x1bd168767b435f84330377d4d5699a086b13322ef3f1d1a7b568c26813caca8d,
                0x2c7cbaad2b912db234378ede551853f07a82f342fe67e6916a32c4d7df2cd6bc
            ]
        );
    }

    function pointBetaH() internal pure returns (Bn254.G2Point memory) {
        return Bn254.G2Point(
            [
                0x28962b931459b7877c4e7b7088a8975508b6d4a1d1e3df25f12425c27eb863bf,
                0x0d7708b73042ffc2dcdf512c47d109002ccb7e16bdf2cfe78250d22135eff87a
            ],
            [
                0x03f8e37e8944bc55a584c53c833feb0c8f99f6f8691617c7324d4b4bbab5b947,
                0x2e8abb48266164ee1e81d725fb34efb2bcac51bea317c3244014c47b6f218107
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
        require(points.length == evals.length, "Array length mismatch");
        require(points.length == openings.length, "Array length mismatch");
        require(points.length == commitments.length, "Array length mismatch");
        
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