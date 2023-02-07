// We require the Hardhat Runtime Environment explicitly here. This is optional
// but useful for running the script in a standalone fashion through `node <script>`.
//
// When running the script with `npx hardhat run <script>` you'll find the Hardhat
// Runtime Environment's members available in the global scope.
import { ethers } from "hardhat";
import { BigNumber } from "ethers"; 

async function main() {
  // Hardhat always runs the compile task when running scripts with its command
  // line interface.
  //
  // If this script is run directly using `node` you may want to call compile
  // manually to make sure everything is compiled
  // await hre.run('compile');

  // We get the contract to deploy
  const Verifier = await ethers.getContractFactory("PoSolVerifier");
  const verifier = await Verifier.deploy();

  await verifier.deployed();

  console.log("PoSol Verifier deployed to:", verifier.address);

  const assetKey = verifier.computeAssetKey("Ethereum");
  await verifier.registerAsset(assetKey);

  const proof = {
    proof: {
      b: BigNumber.from("0x0687b6bdb396a4f50f23a60eb06ac8b8fc1f5386817a401055029126e85f2dd9"),
      t: BigNumber.from("0x1a2b38cc39835a45590e6a407a416788ea9ce82fc681986d095707d033693c82"),
      h1: BigNumber.from("0x2edec743ee1d03581427978ce60cb5fc5cf89ae099133000a87171118e7204b7"),
      h2: BigNumber.from("0x15ba371c75aff1aec34eb647b4f9c3faa8c20cc860cbaff08eaa8e7e6fe43250"),
      sNext: BigNumber.from("0x1d9ce084978ced51f5e33d05e1699b247be5ffd81e84f2111aeab21ee35e2538"),
      zNext: BigNumber.from("0x0abbd001c2ea9f0cf6bde5eeb10b699b3aa72a42cbfd83a5ea5931004af97413"),
      h1Next: BigNumber.from("0x1984993bdfc850f2512475b28c86bbd36e7090ff97e179e041574556b17df270"),
      h2Next: BigNumber.from("0x09ae1ca9af4c2182452b9271f319bcc112fd97becb2f7c85d2340668e09eb9da"),
      bCommit: {
        x: BigNumber.from("0x26904faf3673093ef5b852f09b0dbff42c947f5dddc8f125615b55fd4e3a3390"),
        y: BigNumber.from("0x012fd99abe04ec568c6ff370f9a4e9d97a9d3a6b7d13b075bf21048773e67783"),
      },
      sCommit: {
        x: BigNumber.from("0x000ca200b6132349c7a133592b81fe0820f336f8d3e29a60f0433ed2bb8dabe2"),
        y: BigNumber.from("0x0d9c64f016314f8bec87f405696db0d0f3750c63bdb51f8730369636f3d845ee")
      },
      h1Commit: {
        x: BigNumber.from("0x2f163b6cdd1908686ff2fa1ffd9b49f76cb90afc19b2a4fd42fd0276de486693"),
        y: BigNumber.from("0x1d163fcab184974de9e928a527364379d4aca05b50e76b4a6024521019e31a56")
      },
      h2Commit: {
        x: BigNumber.from("0x28f9f8fc6099321311d41e4d5653b6f6e79c7afd490ce398f7e303f4df723dcc"),
        y: BigNumber.from("0x01648ac86acf4c1f8bd43e18f785bc995c13343e2ef22f7b42d0ab65304e2edd")
      },
      zCommit: {
        x: BigNumber.from("0x28fc2a4bb38e374b7736dd8474a04b0469e8544b46fd630f7d8f8ccd6e250c76"),
        y: BigNumber.from("0x214132644e8f5f03fd6c900a6ff4963ac643d88458587207026e013f1bc46ea0")
      },
      q1Commit: {
        x: BigNumber.from("0x2ca479a3457e631a26459cdd43a0034f1e81e2f30171cf58782dd55b290e923e"),
        y: BigNumber.from("0x08f261b5f0a336c8582905adc14b2e3bca590101a91444e1eb0dd2419d33af41")
      },
      q2Commit: {
        x: BigNumber.from("0x13556ed1045d6cdfe0aa8b6e8c2671102c3ad38b98dc8e4f2490ddbffee58905"),
        y: BigNumber.from("0x187748e4e9fd186e81b4fd816ce0d8fcf56314db1d17dd08902a0569e3dba32b")
      },
      opening1: {
        x: BigNumber.from("0x0af9b395f22c645c5cb086918649feaada9798086e89695544c4209d81b956ea"),
        y: BigNumber.from("0x182e5666a1081d8a8a200af760cb87f4f648254e5d64319d2bc12faab439fc0f")
      },
      opening2: {
        x: BigNumber.from("0x08ccc2ae9c92e753b4387d59abc1ebb015a0eff8c4368ce20ae834b8e928cc3d"),
        y: BigNumber.from("0x102418462006340cf2029bbcba317e991f77a528b5013e2e84abc1094867bdf4")
      }
    },
    balanceSum: BigNumber.from("0x0000000000000000000000000000000000000000000000000000000075c7dc8e")
  };
  const tag = {
    x: BigNumber.from("0x1ab3b724ffd61feb3ecfdbf17a55cc26c0df8841f182adfdaf1ba8990ca3d829"),
    y: BigNumber.from("0x0d83e0b13e6e5d9e1cfcfa060f2f7da003802df6218d885a951a761822d0ccd9")
  };

  await verifier.verifyProof(assetKey, tag, [proof, proof, proof]);
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
