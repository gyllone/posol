// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.0;

library Bn254 {
    uint256 constant private Q_MOD = 21888242871839275222246405745257275088696311157297823662689037894645226208583;
    uint256 constant private R_MOD = 21888242871839275222246405745257275088548364400416034343698204186575808495617;
    uint256 constant private BN254_B_COEFF = 3;

    struct Fr {
        uint256 value;
    }

    struct G1Point {
        uint256 X;
        uint256 Y;
    }

    // Encoding of field elements is: X[0] * z + X[1]
    struct G2Point {
        uint256[2] X;
        uint256[2] Y;
    }

    // function newFr(uint256 fr) internal pure returns (Fr memory) {
    //     return Fr({value: fr});
    // }

    function newCheckedFr(uint256 fr) internal pure returns (Fr memory) {
        require(fr < R_MOD, "Fr is invalid");
        return Fr({value: fr});
    }

    function cloneFr(Fr memory self) internal pure returns (Fr memory) {
        return Fr({value: self.value});
    }

    function copyFromFr(Fr memory self, Fr memory other) internal pure {
        self.value = other.value;
    }

    function inverse(Fr memory fr) internal view returns (Fr memory result) {
        require(fr.value != 0);
        powIntoDest(fr, result, R_MOD - 2);
    }

    function inverseAssign(Fr memory fr) internal view {
        require(fr.value != 0);
        powIntoDest(fr, fr, R_MOD - 2);
    }

    function add(Fr memory self, Fr memory other) internal pure returns (Fr memory) {
        return Fr({value: addmod(self.value, other.value, R_MOD)});
    }

    function addAssign(Fr memory self, Fr memory other) internal pure {
        self.value = addmod(self.value, other.value, R_MOD);
    }

    function sub(Fr memory self, Fr memory other) internal pure returns (Fr memory) {
        return Fr({value: addmod(self.value, R_MOD - other.value, R_MOD)});
    }

    function subAssign(Fr memory self, Fr memory other) internal pure {
        self.value = addmod(self.value, R_MOD - other.value, R_MOD);
    }

    function mul(Fr memory self, Fr memory other) internal pure returns (Fr memory) {
        return Fr({value: mulmod(self.value, other.value, R_MOD)});
    }

    function mulAssign(Fr memory self, Fr memory other) internal pure {
        self.value = mulmod(self.value, other.value, R_MOD);
    }

    function pow(Fr memory self, uint256 power) internal view returns (Fr memory result) {
        powIntoDest(self, result, power);
    }

    function powIntoDest(Fr memory self, Fr memory dest, uint256 power) internal view {
        uint256[6] memory input = [32, 32, 32, self.value, power, R_MOD];
        uint256[1] memory result;
        bool success;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            success := staticcall(gas(), 0x05, input, 0xc0, result, 0x20)
        }
        require(success);
        dest.value = result[0];
    }

    function P1() internal pure returns (G1Point memory) {
        return G1Point(1, 2);
    }

    // function newG1(uint256 x, uint256 y) internal pure returns (G1Point memory) {
    //     return G1Point(x, y);
    // }

    function newCheckedG1(uint256 x, uint256 y) internal pure returns (G1Point memory) {
        if (x == 0 && y == 0) {
            // point of infinity is (0,0)
            return G1Point(x, y);
        }

        // check encoding
        require(x < Q_MOD, "x axis isn't valid");
        require(y < Q_MOD, "y axis isn't valid");
        // check on curve
        uint256 lhs = mulmod(y, y, Q_MOD); // y^2

        uint256 rhs = mulmod(x, x, Q_MOD); // x^2
        rhs = mulmod(rhs, x, Q_MOD); // x^3
        rhs = addmod(rhs, BN254_B_COEFF, Q_MOD); // x^3 + b
        require(lhs == rhs, "is not on curve");

        return G1Point(x, y);
    }

    function copyFromG1(G1Point memory self, G1Point memory other) internal pure {
        self.X = other.X;
        self.Y = other.Y;
    }

    function cloneG1(G1Point memory self) internal pure returns (G1Point memory result) {
        return G1Point(self.X, self.Y);
    }

    function P2() internal pure returns (G2Point memory) {
        // for some reason ethereum expects to have c1*v + c0 form
        return G2Point(
            [
                0x198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c2,
                0x1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed
            ],
            [
                0x090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b,
                0x12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa
            ]
        );
    }

    function pointNegate(G1Point memory self) internal pure returns (G1Point memory result) {
        // The prime q in the base field F_q for G1
        if (self.Y == 0) {
            require(self.X == 0);
        } else {
            result.X = self.X;
            result.Y = Q_MOD - self.Y;
        }
    }

    function pointNegateAssign(G1Point memory self) internal pure {
        // The prime q in the base field F_q for G1
        if (self.Y == 0) {
            require(self.X == 0);
        } else {
            self.Y = Q_MOD - self.Y;
        }
    }

    function pointAdd(G1Point memory p1, G1Point memory p2) internal view returns (G1Point memory r) {
        pointAddIntoDest(p1, p2, r);
        return r;
    }

    function pointAddAssign(G1Point memory p1, G1Point memory p2) internal view {
        pointAddIntoDest(p1, p2, p1);
    }

    function pointAddIntoDest(
        G1Point memory p1,
        G1Point memory p2,
        G1Point memory dest
    ) internal view {
        if (p2.X == 0 && p2.Y == 0) {
            // we add zero, nothing happens
            dest.X = p1.X;
            dest.Y = p1.Y;
            return;
        } else if (p1.X == 0 && p1.Y == 0) {
            // we add into zero, and we add non-zero point
            dest.X = p2.X;
            dest.Y = p2.Y;
            return;
        } else {
            uint256[4] memory input;

            input[0] = p1.X;
            input[1] = p1.Y;
            input[2] = p2.X;
            input[3] = p2.Y;

            bool success;
            // solhint-disable-next-line no-inline-assembly
            assembly {
                success := staticcall(gas(), 6, input, 0x80, dest, 0x40)
            }
            require(success);
        }
    }

    function pointSub(G1Point memory p1, G1Point memory p2) internal view returns (G1Point memory r) {
        pointSubIntoDest(p1, p2, r);
        return r;
    }

    function pointSubAssign(G1Point memory p1, G1Point memory p2) internal view {
        pointSubIntoDest(p1, p2, p1);
    }

    function pointSubIntoDest(G1Point memory p1, G1Point memory p2, G1Point memory dest) internal view {
        if (p2.X == 0 && p2.Y == 0) {
            // we subtracted zero, nothing happens
            dest.X = p1.X;
            dest.Y = p1.Y;
            return;
        } else if (p1.X == 0 && p1.Y == 0) {
            // we subtract from zero, and we subtract non-zero point
            dest.X = p2.X;
            dest.Y = Q_MOD - p2.Y;
            return;
        } else {
            uint256[4] memory input;
            input[0] = p1.X;
            input[1] = p1.Y;
            input[2] = p2.X;
            input[3] = Q_MOD - p2.Y;

            bool success = false;
            // solhint-disable-next-line no-inline-assembly
            assembly {
                success := staticcall(gas(), 6, input, 0x80, dest, 0x40)
            }
            require(success);
        }
    }

    function pointMul(G1Point memory p, Fr memory s) internal view returns (G1Point memory r) {
        pointMulIntoDest(p, s, r);
        return r;
    }

    function pointMulAssign(G1Point memory p, Fr memory s) internal view {
        pointMulIntoDest(p, s, p);
    }

    function pointMulIntoDest(G1Point memory p, Fr memory s, G1Point memory dest) internal view {
        uint256[3] memory input;
        input[0] = p.X;
        input[1] = p.Y;
        input[2] = s.value;
        bool success;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            success := staticcall(gas(), 7, input, 0x60, dest, 0x40)
        }
        require(success);
    }

    function pairing(G1Point[] memory p1, G2Point[] memory p2) internal view returns (bool) {
        require(p1.length == p2.length);
        uint256 elements = p1.length;
        uint256 inputSize = elements * 6;
        uint256[] memory input = new uint256[](inputSize);
        for (uint256 i = 0; i < elements; i++) {
            input[i * 6 + 0] = p1[i].X;
            input[i * 6 + 1] = p1[i].Y;
            input[i * 6 + 2] = p2[i].X[0];
            input[i * 6 + 3] = p2[i].X[1];
            input[i * 6 + 4] = p2[i].Y[0];
            input[i * 6 + 5] = p2[i].Y[1];
        }
        uint256[1] memory out;
        bool success;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            success := staticcall(gas(), 8, add(input, 0x20), mul(inputSize, 0x20), out, 0x20)
        }
        require(success);
        return out[0] != 0;
    }

    /// Convenience method for a pairing check for two pairs.
    function pairingProd2(
        G1Point memory a1,
        G2Point memory a2,
        G1Point memory b1,
        G2Point memory b2
    ) internal view returns (bool) {
        G1Point[] memory p1 = new G1Point[](2);
        G2Point[] memory p2 = new G2Point[](2);
        p1[0] = a1;
        p1[1] = b1;
        p2[0] = a2;
        p2[1] = b2;
        return pairing(p1, p2);
    }
}