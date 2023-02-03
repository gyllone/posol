import { expect } from "chai";
import { ethers } from "hardhat";
import { BigNumber } from "ethers"; 

// describe("SanityCheck", function () {
//   it("Should pass domain check", async function() {
//     const SanityChecker = await ethers.getContractFactory("SanityChecker");
//     const sanity = await SanityChecker.deploy();
//     await sanity.deployed();
    
//     await sanity.checkDomainElement();
//   });

//   it("Should pass g1 affine check", async function() {
//     const SanityChecker = await ethers.getContractFactory("SanityChecker");
//     const sanity = await SanityChecker.deploy();
//     await sanity.deployed();
    
//     await sanity.checkG1Add();
//     await sanity.checkScalarMulG1();
//   })

//   it("Should pass kzg check", async function() {
//     const SanityChecker = await ethers.getContractFactory("SanityChecker");
//     const sanity = await SanityChecker.deploy();
//     await sanity.deployed();
    
//     await sanity.checkKZG();
//     await sanity.checkBatchKZG();
//   });
// });

describe("Check Balance Sum Proof", function() {
  it("Should pass balance sum proof", async function() {
    const proof = {
      b: {
        value: BigNumber.from("0x26afcfb111d46dec5318cdee72b9609e8a4e03229073c21f5c8ba32f0c0ff78c"),
      },
      t: {
        value: BigNumber.from("0x157d7d540175dbaa6fceac623d1dba2ad75f33c455011d1b8cc80325ca37039a"),
      },
      h1: {
        value: BigNumber.from("0x114bb546f266bf8234df924899184878a9db4b31dff13c49ebd4d043294a6019"),
      },
      h2: {
        value: BigNumber.from("0x1c751a67e2d3af796422b62ecfd0cbd3bb1cbf86c8e69ac7fa252d2b1c0c293a"),
      },
      sNext: {
        value: BigNumber.from("0x0ef5cdfcea63bb7000656c615182d52aeec020ef60f169417b6ce4e6101cfaec"),
      },
      zNext: {
        value: BigNumber.from("0x2104accdba2f1744173451194cd32900cc2bc7ab19bf72e68d103cca1dceb14a"),
      },
      h1Next: {
        value: BigNumber.from("0x1908f27d96e9a0bf3df83c8eee5a8cca212de5b4ce2f4c98eaa66b67dca7400a"),
      },
      h2Next: {
        value: BigNumber.from("0x0167a667191dbf7ddca611f803ad2e227dffdcae6bf10d6f0bc4a0c72d06a350"),
      },
      bCommit: {
        x: BigNumber.from("0x1b7b21d8fc850ea38a47978120bff1e4c0a3c092c3e32a51b64401993e9645fc"),
        y: BigNumber.from("0x1a532ddde11802f78bed7c10586caec0c3ef36cfeb34559c1f896319925c2d6c"),
      },
      sCommit: {
        x: BigNumber.from("0x2ce65ab355e1d3c793e5efc07fb5c6fc75aff7c24b140c58abcf61cea87181ec"),
        y: BigNumber.from("0x1463faed773e750923785926feb2eca7711b5063ca837bfa8b3c842b216a6b7d"),
      },
      h1Commit: {
        x: BigNumber.from("0x079fb83fdc8fd17ba426ea2c00e3691365b7ef5959b9df40b4ae73e413ddf6a7"),
        y: BigNumber.from("0x1eac736966f74342c116a506bc1c08e4ecf1556296e6ef2b5747177c2e81a2f9"),
      },
      h2Commit: {
        x: BigNumber.from("0x0db51b5cbd5abdbac6c5d92ebaa625316f1ff401847ef390ff343af193926eed"),
        y: BigNumber.from("0x2d96ee2710e7cfc1e0e7c9aef3e6aa89e3416d6a3c25ba06d8315c2a1cccfce2"),
      },
      zCommit: {
        x: BigNumber.from("0x0b1042ec618b3a73a3766ddd1fb878a9a4b0551078a5f327015ec62dddd422a3"),
        y: BigNumber.from("0x107e393c597a7afa93b9ae9f9757bd23ba6d5e0c1fdc6a13d74dbc14f4e7ef2a"),
      },
      q1Commit: {
        x: BigNumber.from("0x304a145f10b0abb609730bbba8e0ff47b6a5196387b70c81342892b8fd8a6bb4"),
        y: BigNumber.from("0x2ca4e34e86da73e98edd3cd632d8c2da2c20832c48b104e43504a6b8d5eae645"),
      },
      q2Commit: {
        x: BigNumber.from("0x06f6c9712f49beec55695438de95365d59c87cfd8e332e538f02de8525a81d5c"),
        y: BigNumber.from("0x194062ba25d1bdee57d3be95b5726f8d6a61a14f9db26f53bbe4be7718f9e243"),
      },
      opening1: {
        x: BigNumber.from("0x0eec4d23b86e54443e0bc7300d8a9f5d339236e3eeb057c311a8fa478b52f34f"),
        y: BigNumber.from("0x2f3892c9be613a59067a9ce7ca743f475238491e983f192373baf08ea7f2740c"),
      },
      opening2: {
        x: BigNumber.from("0x1b2dbe8742e75fe3d9127c6c5768083dee83f36aab49275da9eb10af69b3c5a6"),
        y: BigNumber.from("0x1c8a98e14aa457ead304309b7661f638d27d25ad2d888641842fb10181cdf659"),
      },
    };
    const m = {
      value: BigNumber.from("0x0000000000000000000000000000000000000000000000000000000075c7dc8e"),
    };

    const PoSolVerifier = await ethers.getContractFactory("PoSolVerifier");
    const verifier = await PoSolVerifier.deploy();
    await verifier.deployed();

    expect(await verifier.verifyBalanceSum(proof, m)).to.be.equal(true);
  });
});
