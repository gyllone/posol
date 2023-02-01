import { expect } from "chai";
import { ethers } from "hardhat";

describe("SanityCheck", function () {
  it("Should pass domain check", async function() {
    const SanityChecker = await ethers.getContractFactory("SanityChecker");
    const sanity = await SanityChecker.deploy();
    await sanity.deployed();
    
    await sanity.checkDomainElement();
  });

  it("Should pass g1 affine check", async function() {
    const SanityChecker = await ethers.getContractFactory("SanityChecker");
    const sanity = await SanityChecker.deploy();
    await sanity.deployed();
    
    await sanity.checkG1Add();
    await sanity.checkScalarMulG1();
  })

  it("Should pass kzg check", async function() {
    const SanityChecker = await ethers.getContractFactory("SanityChecker");
    const sanity = await SanityChecker.deploy();
    await sanity.deployed();
    
    await sanity.checkKZG();
    await sanity.checkBatchKZG();
  });
});
