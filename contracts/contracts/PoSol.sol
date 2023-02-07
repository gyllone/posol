//SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.0;
pragma abicoder v2;

import "./library/Bn254.sol";
import "./library/Domain.sol";
import "./library/KZGChecker.sol";
import "./library/BalanceSumVerifier.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract PoSolVerifier is Ownable {
    using Bn254 for Bn254.Fr;
    using Bn254 for Bn254.G1Point;
    using BalanceSumVerifier for BalanceSumVerifier.Proof;

    struct BalanceSumStamp {
        uint32 timestamp;
        uint256 balanceSum;
    }

    struct BalanceSumProof {
        BalanceSumVerifier.Proof proof;
        Bn254.Fr balanceSum;
    }

    struct CommittedData {
        uint224 maxBalance;
        BalanceSumStamp balanceSumStamp;
        Bn254.G1Point aggBalanceCommit;
        Bn254.G1Point tagCommit;
    }

    struct CommittedAsset {
        bool supported;
        CommittedData[] committedData;
    }

    mapping(bytes32 => CommittedAsset) private committedAssets;

    function computeAssetKey(string memory name) external pure returns (bytes32) {
        return keccak256(bytes(name));
    }

    function computeUserTag(
        string memory id,
        string memory salt
    ) external pure returns (Bn254.Fr memory) {
        uint256 tag = uint256(keccak256(abi.encodePacked(id, salt)));
        return Bn254.Fr(tag & 0x1fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff);
    }

    function getBalanceSum(
        bytes32 assetKey,
        uint256 dataIndex
    ) external view returns (BalanceSumStamp memory) {
        CommittedAsset storage asset = committedAssets[assetKey];
        require(asset.supported, "Asset is not supported");
        require(dataIndex < asset.committedData.length, "Data index out of range");
        return asset.committedData[dataIndex].balanceSumStamp;
    }

    function getBalanceSumFromRange(
        bytes32 assetKey,
        uint256 start,
        uint256 end
    ) external view returns (BalanceSumStamp[] memory) {
        CommittedAsset storage asset = committedAssets[assetKey];
        require(asset.supported, "Asset is not supported");
        require(end < asset.committedData.length, "Data index out of range");
        require(start < end, "Start is not less than end");

        BalanceSumStamp[] memory balanceSumStamp = new BalanceSumStamp[](end - start);
        for (uint256 i = start; i < end; i++) {
            balanceSumStamp[i - start] = asset.committedData[i].balanceSumStamp;
        }
        return balanceSumStamp;
    }

    function individualVerify(
        bytes32 assetKey,
        uint256 dataIndex,
        uint256 userIndex,
        uint224 balance,
        Bn254.Fr memory tag,
        Bn254.G1Point memory tagOpening,
        Bn254.G1Point memory balanceOpening
    ) external view {
        CommittedAsset storage asset = committedAssets[assetKey];
        require(asset.supported, "Asset is not supported");
        require(dataIndex < asset.committedData.length, "Data index out of range");
        require(userIndex < Domain.SIZE, "User index out of range");
        require(tag.isFrValid(), "Tag is invalid");
        require(tagOpening.isG1Valid(), "Tag opening is invalid");
        require(balanceOpening.isG1Valid(), "Balance opening is invalid");

        CommittedData memory data = asset.committedData[dataIndex];
        require(balance <= data.maxBalance, "Balance is too large");

        Bn254.Fr memory point = Domain.element(userIndex);
        Bn254.Fr memory balanceFr = Bn254.Fr(balance);
        // check tag opening
        bool result = KZGChecker.check(point, tag, tagOpening, data.tagCommit);
        require(result, "Tag verification failed");
        // check balance opening
        result = KZGChecker.check(point, balanceFr, balanceOpening, data.aggBalanceCommit);
        require(result, "Balance verification failed");
    }

    function registerAsset(bytes32 assetKey) external onlyOwner {
        CommittedAsset storage asset = committedAssets[assetKey];
        require(!asset.supported, "Asset is already supported");
        asset.supported = true;
    }

    function verifyProof(
        bytes32 assetKey,
        Bn254.G1Point memory tagCommit,
        BalanceSumProof[] memory proofs
    ) external onlyOwner {
        CommittedAsset storage asset = committedAssets[assetKey];
        require(asset.supported, "Asset is not supported");
        require(proofs.length > 0, "No balance sum proof provided");

        Bn254.G1Point memory aggBalanceCommit = Bn254.G1Point(0, 0);
        Bn254.Fr memory aggBalanceSum = Bn254.Fr(0);
        Bn254.Fr memory multiplier = Bn254.Fr(1);
        Bn254.Fr memory domainSize = Bn254.Fr(Domain.SIZE);
        for (uint i = 0; i < proofs.length; i++) {
            BalanceSumProof memory proof = proofs[i];
            // verify each balance sum proof
            bool result = proof.proof.verify(proof.balanceSum);
            require(result, "Failed verify balance sum proof");

            // aggregate balance sum
            proof.balanceSum.mulAssign(multiplier);
            aggBalanceSum.addAssign(proof.balanceSum);
            // aggregate balance commitment
            proof.proof.bCommit.pointMulAssign(multiplier);
            aggBalanceCommit.pointAddAssign(proof.proof.bCommit);
            // update multiplier
            multiplier.mulAssign(domainSize);
        }

        BalanceSumStamp memory balanceSumStamp = BalanceSumStamp({
            // solhint-disable-next-line not-rely-on-time
            timestamp: uint32(block.timestamp),
            balanceSum: aggBalanceSum.value
        });
        asset.committedData.push(CommittedData({
            balanceSumStamp: balanceSumStamp,
            maxBalance: uint224(Domain.SIZE) ** uint8(proofs.length),
            aggBalanceCommit: aggBalanceCommit,
            tagCommit: tagCommit
        }));
    }
}
