import { expect } from "chai";
import { ethers } from "hardhat";

describe("KZGChecker", function () {
  it("Should pass kzg checking", async function () {
    const Greeter = await ethers.getContractFactory("KZG");
    const greeter = await Greeter.deploy("Hello, world!");
    await greeter.deployed();

    expect(await greeter.greet()).to.equal("Hello, world!");

    const setGreetingTx = await greeter.setGreeting("Hola, mundo!");

    // wait until the transaction is mined
    await setGreetingTx.wait();

    expect(await greeter.greet()).to.equal("Hola, mundo!");
  });
});
