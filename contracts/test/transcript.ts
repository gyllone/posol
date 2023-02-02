import { expect } from "chai";
import { ethers } from "hardhat";
import { BigNumber } from "ethers"; 

describe("Transcript", function () {
  it("Should pass", async function() {
    const MockTranscript = await ethers.getContractFactory("MockTranscript");
    const transcript = await MockTranscript.deploy();
    await transcript.deployed();
    
    const item1 = 1;
    const item2 = { value: BigNumber.from(2) };
    const item3 = { x: BigNumber.from(3), y: BigNumber.from(4) };
    var [c1, c2, c3] = await transcript.testTranscript(item1, item2, item3);
    expect(c1).to.be.equal(BigNumber.from("0x0f9d11cec4f06b0d18060cde3db4196495ddfbb096108951446fc8a1d45f4b59"));
    expect(c2).to.be.equal(BigNumber.from("0x0f4dccb919a5dba2dd010a562ba45b4551291f5e565706536e78b24ac8b5c64d"));
    expect(c3).to.be.equal(BigNumber.from("0x1b5bf46adfcd1dd4f9ac7166586cf83f261192bc4b83fdda30ddee22f9054c1f"));
  });
});
