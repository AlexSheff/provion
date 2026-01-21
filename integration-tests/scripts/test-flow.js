const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("Provion Integration Flow", function () {
  it("Should deploy MockUSDC", async function () {
    const MockUSDC = await ethers.getContractFactory("MockUSDC");
    const musdc = await MockUSDC.deploy();
    await musdc.waitForDeployment();
    expect(await musdc.name()).to.equal("Mock USDC");
  });
});
