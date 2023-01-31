// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.0;

import "./Bn254.sol";

library KZGChecker {
    using Bn254 for Bn254.Fr;
    using Bn254 for Bn254.G1Point;

    // solhint-disable-next-line func-name-mixedcase
    function X2() internal pure returns (Bn254.G2Point memory) {
        return Bn254.G2Point(
            [
                0x250509b6fd346fb4986ceb3e3c2ae1352b91a2899c44a9bff7347f9309d5b2bb,
                0x0205238886dab0bbd5326f37072d4e59033807777f1dd0ac69b6a576d4298df7
            ],
            [
                0x0850c0444822f06b1f53da68c3ff950b1864b1982df33d23ea342ba16b713917,
                0x28d625b881fd1da41d022112823ad7e55d43259bb37cd1bf7e08838fb5c35a7d
            ]
        );
    }

    function check(
        Bn254.Fr memory point,
        Bn254.Fr memory eval,
        Bn254.G1Point memory opening,
        Bn254.G1Point memory commitment
    ) internal view returns (bool) {
        Bn254.G1Point memory p1 = Bn254.P1();
        Bn254.G2Point memory p2 = Bn254.P2();
        Bn254.G2Point memory x2 = X2();

        Bn254.G1Point memory g1 = p1.pointMul(eval);
        g1.pointSubAssign(commitment);
        g1.pointSubAssign(opening.pointMul(point));

        return Bn254.pairingProd2(opening, x2, g1, p2);
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
        
        Bn254.G1Point memory p1 = Bn254.P1();
        Bn254.G2Point memory p2 = Bn254.P2();
        Bn254.G2Point memory x2 = X2();

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
            tmpG1.copyFromG1(p1);
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
        return Bn254.pairingProd2(partA, x2, partB, p2);
    }
}