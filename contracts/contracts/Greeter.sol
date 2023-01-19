//SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.0;

import "./library/Bn254.sol";
import "./library/BalanceSumVerifier.sol";

contract Greeter {

    // constructor(string memory _greeting) {
    // }

    function verify(
        BalanceSumVerifier.Proof memory proof,
        Bn254.Fr memory balanceSum
    ) public view returns (bool) {
        return BalanceSumVerifier.verify(proof, balanceSum);
    }
}
