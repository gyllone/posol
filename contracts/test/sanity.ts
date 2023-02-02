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
        value: BigNumber.from("0x28319fc47334e5452b7460ff479f79282bc1474958a1625248e34af5a6da1764"),
      },
      t: {
        value: BigNumber.from("0x2902b082b9aa5203112b8d743f22718bc6921e5fd8cb68cd2335a8a28d18ec4e"),
      },
      h1: {
        value: BigNumber.from("0x17acae83bd297d5e2e1697fd57bb30d35db7f41314e8af8fae679ede39672b66"),
      },
      h2: {
        value: BigNumber.from("0x17d054bffe2b63828c312b283cea8dc0fd9acb6faf7f061963dc3fb1aefe6cbb"),
      },
      sNext: {
        value: BigNumber.from("0x11743a905a1963b413cf828d42c50d5d16d7c0a87f2b993b590e92734a198cd9"),
      },
      zNext: {
        value: BigNumber.from("0x2469e6293cc96a0f11260c3dd5dac7b0f52f6b3e9b756611f3b5dc6c59a3801f"),
      },
      h1Next: {
        value: BigNumber.from("0x2c5716b7ced3203f648805c1f9cee83b511100b49c0a9622b90bd3aaf7b7672e"),
      },
      h2Next: {
        value: BigNumber.from("0x07b38a4128c2778acc83a6b3c9ab7a91b8806cd028f0d111c78b00923443b870"),
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
        x: BigNumber.from("0x08fb8d334c7b87d467f5b07c466248a5ea5c3c025a386037b27f6c8bb59af531"),
        y: BigNumber.from("0x13aef4dbbd54ad73db005731e169a49cfccc03a30718778b4692f62904d88a49"),
      },
      q1Commit: {
        x: BigNumber.from("0x21d581f38b6910b54cb499d0f1b5ff95858d2e015235affcd8f5f0eefaf1c437"),
        y: BigNumber.from("0x02a8b0c4058ab5e0309db277106b4d091825388cf03608cf65f921fea8937d96"),
      },
      q2Commit: {
        x: BigNumber.from("0x2433f05f83ceb701c7b6d4152fd3c59f6470020f7f39183aca40b6b276eeadf4"),
        y: BigNumber.from("0x0c0cc540a47b5d227fefe7dc8d5edb4104f716a41393d8df04a3438d5af2bc0a"),
      },
      opening1: {
        x: BigNumber.from("0x0519a1b094706c33f7a7f4dc60e1643025e1adfb66d1ad61035a632180875392"),
        y: BigNumber.from("0x1092e5cffd177431128bd5ae17fd99a11e8279dd4ae46cb0e294082d38be7984"),
      },
      opening2: {
        x: BigNumber.from("0x03aebcac21653cd9a67be0a4cf89e34d6bd280f092e6086eb115f685c65744ae"),
        y: BigNumber.from("0x2ca6fd563bd039d8d412dd884cda9d9f3e226847b7374774a0e40edf501bcae6"),
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
