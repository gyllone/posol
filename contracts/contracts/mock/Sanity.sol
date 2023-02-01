//SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.0;

import "../library/Bn254.sol";
import "../library/Domain.sol";
import "../library/KZGChecker.sol";

contract SanityChecker {
    using Bn254 for Bn254.G1Point;
    
    function checkDomainElement() external view {
        Bn254.Fr memory point = Domain.element(10000);
        require(
            point.value == 0x0b27a6d499620ebed68727cba6565bcd869c7a13ced628604eaa499b5c059690,
            "Domain sanity check failed"
        );
    }

    function checkG1() public view {
        Bn254.G1Point memory a = Bn254.G1Point(
            0x08619b0c2bc95ba3b5e6f720d4f5bcfceb62d455173d554ab7cefaaa4b42f09f,
            0x13fc65dd4e5c6b90d42b8741fe2821e82c0bd8a6d5a43d27a5221d9313f27d48
        );

        Bn254.G1Point memory b = a.pointAdd(a);
        require(b.x == 0x2255a5418639a5da467b6e0927112f4674b22cda6856c5027761ffa0332bbc60, "x wrong");
        require(b.y == 0x0c640b679d694bad015d248b92f46f13c6974c0b694678d7fa75ecea015631d3, "y wrong");
    }

    function checkScalarMulG1() public view {
        Bn254.Fr memory point = Domain.element(10000);
        Bn254.G1Point memory a = Bn254.G1Point(
            0x08619b0c2bc95ba3b5e6f720d4f5bcfceb62d455173d554ab7cefaaa4b42f09f,
            0x13fc65dd4e5c6b90d42b8741fe2821e82c0bd8a6d5a43d27a5221d9313f27d48
        );
        Bn254.G1Point memory b = a.pointMul(point);
        require(b.x == 0x1830550faf26dd9e3a83041bec6398d7b7e3514360b591844e98788b5dc148e8, "x wrong");
        require(b.y == 0x17cd4b82c4d7658ceea05d2007425223c094d1b1bb24b26c48f5f95c91299786, "y wrong");
    }

    function checkKZG() public view {
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

    function checkBatchKZG() public view {
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

        Bn254.Fr[] memory points = new Bn254.Fr[](2);
        points[0] = point;
        points[1] = point;

        Bn254.Fr[] memory evals = new Bn254.Fr[](2);
        evals[0] = eval;
        evals[1] = eval;

        Bn254.G1Point[] memory commitments = new Bn254.G1Point[](2);
        commitments[0] = commitment;
        commitments[1] = commitment;

        Bn254.G1Point[] memory openings = new Bn254.G1Point[](2);
        openings[0] = opening;
        openings[1] = opening;

        bool result = KZGChecker.batchCheck(eval, points, evals, openings, commitments);
        require(result, "KZG batch check failed");
    }
}
