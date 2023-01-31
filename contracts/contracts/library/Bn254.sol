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
        uint256 x;
        uint256 y;
    }

    // Encoding of field elements is: X[0] * z + X[1]
    struct G2Point {
        uint256[2] x;
        uint256[2] y;
    }

    function validateFr(Fr memory self) internal pure {
        require(self.value < R_MOD, "Fr is invalid");
    }

    function cloneFr(Fr memory self) internal pure returns (Fr memory) {
        return Fr({value: self.value});
    }

    function copyFromFr(Fr memory self, Fr memory other) internal pure {
        self.value = other.value;
    }

    function inverse(Fr memory fr) internal view returns (Fr memory result) {
        require(fr.value != 0, "Fr is zero");
        powIntoDest(fr, result, R_MOD - 2);
    }

    function inverseAssign(Fr memory fr) internal view {
        require(fr.value != 0, "Fr is zero");
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
        bool success;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            success := staticcall(gas(), 0x05, input, 0xc0, dest, 0x20)
        }
        require(success, "Fr pow operation failed");
    }

    function validateG1(G1Point memory self) internal pure {
        if (self.x == 0 && self.y == 0) {
            return;
        }

        // check encoding
        require(self.x < Q_MOD, "X axis isn't valid");
        require(self.y < Q_MOD, "Y axis isn't valid");
        // check on curve
        uint256 lhs = mulmod(self.y, self.y, Q_MOD); // y^2

        uint256 rhs = mulmod(self.x, self.x, Q_MOD); // x^2
        rhs = mulmod(rhs, self.x, Q_MOD); // x^3
        rhs = addmod(rhs, BN254_B_COEFF, Q_MOD); // x^3 + b
        require(lhs == rhs, "G1 point is not on curve");
    }

    function copyFromG1(G1Point memory self, G1Point memory other) internal pure {
        self.x = other.x;
        self.y = other.y;
    }

    function cloneG1(G1Point memory self) internal pure returns (G1Point memory result) {
        return G1Point(self.x, self.y);
    }

    function pointNegate(G1Point memory self) internal pure returns (G1Point memory result) {
        // The prime q in the base field F_q for G1
        if (self.y == 0) {
            require(self.x == 0, "Invalid G1 point");
        } else {
            result.x = self.x;
            result.y = Q_MOD - self.y;
        }
    }

    function pointNegateAssign(G1Point memory self) internal pure {
        // The prime q in the base field F_q for G1
        if (self.y == 0) {
            require(self.x == 0, "Invalid G1 point");
        } else {
            self.y = Q_MOD - self.y;
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
        if (p2.x == 0 && p2.y == 0) {
            // we add zero, nothing happens
            dest.x = p1.x;
            dest.y = p1.y;
            return;
        } else if (p1.x == 0 && p1.y == 0) {
            // we add into zero, and we add non-zero point
            dest.x = p2.x;
            dest.y = p2.y;
            return;
        } else {
            uint256[4] memory input;
            input[0] = p1.x;
            input[1] = p1.y;
            input[2] = p2.x;
            input[3] = p2.y;

            bool success;
            // solhint-disable-next-line no-inline-assembly
            assembly {
                success := staticcall(gas(), 0x06, input, 0x80, dest, 0x40)
            }
            require(success, "G1 point addition failed");
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
        if (p2.x == 0 && p2.y == 0) {
            // we subtracted zero, nothing happens
            dest.x = p1.x;
            dest.y = p1.y;
            return;
        } else if (p1.x == 0 && p1.y == 0) {
            // we subtract from zero, and we subtract non-zero point
            dest.x = p2.x;
            dest.y = Q_MOD - p2.y;
            return;
        } else {
            uint256[4] memory input;
            input[0] = p1.x;
            input[1] = p1.y;
            input[2] = p2.x;
            input[3] = Q_MOD - p2.y;

            bool success = false;
            // solhint-disable-next-line no-inline-assembly
            assembly {
                success := staticcall(gas(), 0x06, input, 0x80, dest, 0x40)
            }
            require(success, "G1 point subtraction failed");
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
        input[0] = p.x;
        input[1] = p.y;
        input[2] = s.value;
        bool success;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            success := staticcall(gas(), 0x07, input, 0x60, dest, 0x40)
        }
        require(success, "G1 point multiplication failed");
    }

    function pairing(G1Point[] memory p1, G2Point[] memory p2) internal view returns (bool) {
        require(p1.length == p2.length, "Unmatched array length");
        uint256 elements = p1.length;
        uint256 inputSize = elements * 6;
        uint256[] memory input = new uint256[](inputSize);
        for (uint256 i = 0; i < elements; i++) {
            input[i * 6 + 0] = p1[i].x;
            input[i * 6 + 1] = p1[i].y;
            input[i * 6 + 2] = p2[i].x[0];
            input[i * 6 + 3] = p2[i].x[1];
            input[i * 6 + 4] = p2[i].y[0];
            input[i * 6 + 5] = p2[i].y[1];
        }
        uint256[1] memory out;
        bool success;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            success := staticcall(gas(), 0x08, add(input, 0x20), mul(inputSize, 0x20), out, 0x20)
        }
        require(success, "Pairing check failed");
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