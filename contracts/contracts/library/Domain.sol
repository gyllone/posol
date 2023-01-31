// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.0;

import "./Bn254.sol";

library Domain {
    uint256 constant public SIZE = 2 ** 27;

    function domainGenerator() internal pure returns (Bn254.Fr memory) {
        return Bn254.Fr(0x049ae702b363ebe85f256a9f6dc6e364b4823532f6437da2034afc4580928c44);
    }

    function domainGeneratorInv() internal pure returns (Bn254.Fr memory) {
        return Bn254.Fr(0x110BF78F435A46E97746A25A15D27CED4C787F72C9718F6CA8B64BA8980DB869);
    }
}