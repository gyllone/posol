//SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.0;
pragma abicoder v2;

import "./library/Bn254.sol";
import "./library/BalanceSumVerifier.sol";

contract PoSolVerifier {
    function verifyBalanceSum(
        BalanceSumVerifier.Proof memory proof,
        Bn254.Fr memory balanceSum
    ) public view returns (bool) {
        return BalanceSumVerifier.verify(proof, balanceSum);
    }
}
