import { expect } from "chai";

// These imports are intentionally identical to the original test location.
// Run this file after placing it under `test/guardian/` in the matching GMX
// Synthetics checkout.
import { deployFixture } from "../../utils/fixture";
import { expandDecimals } from "../../utils/math";
import * as keys from "../../utils/keys";

// PUBLIC ADAPTATION of the original GMX testVaultResiduals.ts.
// This preserves the original fixture and accounting assertions while stopping
// before the subsequent request/capture sequence.
describe("Guardian.VaultResiduals accounting observations (adapted)", () => {
  let fixture;
  let user0, user1;
  let dataStore, router, exchangeRouter, orderVault, depositVault, usdc;

  beforeEach(async () => {
    fixture = await deployFixture();
    ({ user0, user1 } = fixture.accounts);
    ({ dataStore, router, exchangeRouter, orderVault, depositVault, usdc } = fixture.contracts);
  });

  it("observes physical OrderVault balance independently of tokenBalances", async () => {
    const strandedAmount = expandDecimals(2_500, 6);
    await dataStore.setUint(keys.REQUEST_EXPIRATION_TIME, 1);
    await usdc.mint(user0.address, strandedAmount);
    await usdc.connect(user0).approve(router.address, strandedAmount);
    await exchangeRouter.connect(user0).sendTokens(usdc.address, orderVault.address, strandedAmount);

    expect(await usdc.balanceOf(orderVault.address)).eq(strandedAmount);
    expect(await orderVault.tokenBalances(usdc.address)).eq(0);
    expect(await usdc.balanceOf(user1.address)).eq(0);
  });

  it("observes physical DepositVault balance independently of tokenBalances", async () => {
    const strandedAmount = expandDecimals(2_500, 6);
    await dataStore.setUint(keys.REQUEST_EXPIRATION_TIME, 1);
    await usdc.mint(user0.address, strandedAmount);
    await usdc.connect(user0).approve(router.address, strandedAmount);
    await exchangeRouter.connect(user0).sendTokens(usdc.address, depositVault.address, strandedAmount);

    expect(await usdc.balanceOf(depositVault.address)).eq(strandedAmount);
    expect(await depositVault.tokenBalances(usdc.address)).eq(0);
    expect(await usdc.balanceOf(user1.address)).eq(0);
  });
});
