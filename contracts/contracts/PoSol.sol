//SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.0;
pragma abicoder v2;

import "./library/Bn254.sol";
import "./library/Domain.sol";
import "./library/KZGChecker.sol";
import "./library/BalanceSumVerifier.sol";

contract PoSolVerifier {
    using Bn254 for Bn254.Fr;
    using Bn254 for Bn254.G1Point;

    struct BalanceSumStamp {
        uint256 timestamp;
        uint256 balanceSum;
    }

    struct CommittedData {
        uint256 timestamp;
        Bn254.G1Point tagCommit;
        Bn254.G1Point balanceCommit;
        Bn254.Fr balanceSum;
    }

    CommittedData[] private committedData;

    function verifyBalanceSum(
        Bn254.G1Point memory tagCommit,
        BalanceSumVerifier.Proof memory proof,
        Bn254.Fr memory balanceSum
    ) external {
        require(
            BalanceSumVerifier.verify(proof, balanceSum),
            "Balance sum verification failed"
        );

        committedData.push(CommittedData({
            // solhint-disable-next-line not-rely-on-time
            timestamp: block.timestamp,
            tagCommit: tagCommit,
            balanceCommit: proof.bCommit,
            balanceSum: balanceSum
        }));
    }

    function individualVerify(
        uint256 dataIndex,
        uint256 userIndex,
        Bn254.Fr memory tag,
        Bn254.Fr memory balance,
        Bn254.G1Point memory tagOpening,
        Bn254.G1Point memory balanceOpening
    ) external view {
        require(dataIndex < committedData.length, "Data index out of range");
        require(userIndex < Domain.SIZE, "User index out of range");
        require(tag.isFrValid(), "Tag is invalid");
        require(balance.isFrValid(), "Balance is invalid");
        require(tagOpening.isG1Valid(), "Tag opening is invalid");
        require(balanceOpening.isG1Valid(), "Balance opening is invalid");

        CommittedData memory data = committedData[dataIndex];
        Bn254.Fr memory point = Domain.element(userIndex);
        
        // check tag opening
        require(
            KZGChecker.check(point, tag, tagOpening, data.tagCommit),
            "Tag verification failed"
        );
        // check balance opening
        require(
            KZGChecker.check(point, balance, balanceOpening, data.balanceCommit),
            "Balance verification failed"
        );
    }

    function getBalanceSum(uint256 dataIndex) external view returns (BalanceSumStamp memory) {
        require(dataIndex < committedData.length, "Data index out of range");
        CommittedData memory data = committedData[dataIndex];
        return BalanceSumStamp({
            timestamp: data.timestamp,
            balanceSum: data.balanceSum.value
        });
    }

    function getBalanceSumFromRange(
        uint256 start,
        uint256 end
    ) external view returns (BalanceSumStamp[] memory) {
        require(end < committedData.length, "Data index out of range");
        require(start < end, "Start is not less than end");
        BalanceSumStamp[] memory result = new BalanceSumStamp[](end - start);
        for (uint256 i = start; i < end; i++) {
            CommittedData memory data = committedData[i];
            result[i] = BalanceSumStamp({
                timestamp: data.timestamp,
                balanceSum: data.balanceSum.value
            });
        }
        return result;
    }
}
