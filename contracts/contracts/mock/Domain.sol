//SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.0;

import "../library/Bn254.sol";
import "../library/Domain.sol";

contract DomainMocker {
    function sanityCheck() external view {
        Bn254.Fr memory point = Domain.element(10000);
        require(
            point.value == 0x0b27a6d499620ebed68727cba6565bcd869c7a13ced628604eaa499b5c059690,
            "Domain sanity check failed"
        );
    }
}