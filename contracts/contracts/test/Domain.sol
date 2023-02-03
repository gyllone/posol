// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.0;

import "../library/Bn254.sol";

library TestDomain {
    using Bn254 for Bn254.Fr;

    uint64 constant public SIZE = 2 ** 20;

    function domainGenerator() internal pure returns (Bn254.Fr memory) {
        return Bn254.Fr(0x26125da10a0ed06327508aba06d1e303ac616632dbed349f53422da953337857);
    }

    function domainGeneratorInv() internal pure returns (Bn254.Fr memory) {
        return Bn254.Fr(0x100c332d2100895fab6473bc2c51bfca521f45cb3baca6260852a8fde26c91f3);
    }

    function element(uint256 index) internal view returns (Bn254.Fr memory) {
        require(index < SIZE, "index out of range");

        if (index == 0) {
            return Bn254.Fr(1);
        } else if (index == 1) {
            return domainGenerator();
        } else if (index == 2) {
            Bn254.Fr memory omega = domainGenerator();
            return omega.mul(omega);
        } else if (index == SIZE - 1) {
            return domainGeneratorInv();
        } else {
            return domainGenerator().pow(index);
        }
    }
}