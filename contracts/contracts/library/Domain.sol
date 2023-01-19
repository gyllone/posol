// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.0;

import "./Bn254.sol";

library Domain {
    uint256 constant public SIZE = 2 ** 26;

    function domainGenerator() internal pure returns (Bn254.Fr memory) {
        return Bn254.Fr(0x0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0);
    }

    function domainGeneratorInv() internal pure returns (Bn254.Fr memory) {
        return Bn254.Fr(0x0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0e0);
    }
}