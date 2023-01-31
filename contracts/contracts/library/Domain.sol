// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.0;

import "./Bn254.sol";

library Domain {
    using Bn254 for Bn254.Fr;

    uint64 constant public SIZE = 2 ** 27;

    function domainGenerator() internal pure returns (Bn254.Fr memory) {
        return Bn254.Fr(0x049ae702b363ebe85f256a9f6dc6e364b4823532f6437da2034afc4580928c44);
    }

    function domainGeneratorInv() internal pure returns (Bn254.Fr memory) {
        return Bn254.Fr(0x110BF78F435A46E97746A25A15D27CED4C787F72C9718F6CA8B64BA8980DB869);
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