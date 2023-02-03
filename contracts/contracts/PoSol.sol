//SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.0;
pragma abicoder v2;

import "./library/Bn254.sol";
import "./library/BalanceSumVerifier.sol";

contract PoSolVerifier {

    struct VerifiedData {
        uint256 timestamp;
        Bn254.G1Point tagCommit;
        Bn254.G1Point balanceCommit;
        Bn254.Fr balanceSum;
    }

    VerifiedData[] public verifiedData;

    function proveAndCommit(
        Bn254.G1Point memory tagCommit,
        BalanceSumVerifier.Proof memory proof,
        Bn254.Fr memory balanceSum
    ) public {
        bool verified = BalanceSumVerifier.verify(proof, balanceSum);
        require(verified, "Balance sum verification failed");

        verifiedData.push(VerifiedData({
            timestamp: block.timestamp,
            tagCommit: tagCommit,
            balanceCommit: proof.bCommit,
            balanceSum: balanceSum
        }));
    }

    function
}
