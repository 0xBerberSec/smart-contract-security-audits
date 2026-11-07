// @ts-nocheck
import { expect } from "chai";
const { ethers } = require("hardhat");

describe("SANITIZED_DERIVATION: refund accounting", function () {
  async function fixture() {
    const [user] = await ethers.getSigners();
    const Ledger = await ethers.getContractFactory("RefundLedger");
    const ledger = await Ledger.deploy();
    await ledger.waitForDeployment();
    return { user, ledger };
  }

  it("refunds the complete listed balance", async function () {
    const { user, ledger } = await fixture();
    await ledger.deposit(100);
    await ledger.refundListed(user.address, 100, true);
    expect(await ledger.balances(ledger.target)).to.equal(0);
    expect(await ledger.balances(user.address)).to.equal(100);
  });

  it("does not refund an omitted asset", async function () {
    const { user, ledger } = await fixture();
    await ledger.deposit(60);
    await ledger.refundListed(user.address, 60, false);
    expect(await ledger.balances(ledger.target)).to.equal(60);
    expect(await ledger.balances(user.address)).to.equal(0);
  });

  it("rolls back a balance update when the route reverts", async function () {
    const { ledger } = await fixture();
    await expect(ledger.depositThenRevert(25)).to.be.revertedWith("route reverted");
    expect(await ledger.balances(ledger.target)).to.equal(0);
  });
});
