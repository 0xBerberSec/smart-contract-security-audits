import { expect } from "chai";

// These imports are intentionally identical to the original test location.
// Run this file after placing it under `test/guardian/` in the matching GMX
// Synthetics checkout.
import { deployFixture } from "../../utils/fixture";
import { expandDecimals } from "../../utils/math";

// PUBLIC ADAPTATION of the original GMX testExternalHandlerValidation.ts.
// The private original is preserved unchanged. Only non-sensitive control paths
// are included: explicit refund coverage and multicall rollback.
describe("Guardian.ExternalHandler public controls (adapted)", () => {
  let fixture;
  let user0;
  let usdc, router, exchangeRouter, externalHandler, orderVault;

  beforeEach(async () => {
    fixture = await deployFixture();
    ({ user0 } = fixture.accounts);
    ({ usdc, router, exchangeRouter, externalHandler, orderVault } = fixture.contracts);
  });

  it("refunds the residual input token when explicitly listed", async () => {
    const amount = expandDecimals(25, 6);
    await usdc.mint(user0.address, amount);
    await usdc.connect(user0).approve(router.address, amount);
    const before = await usdc.balanceOf(user0.address);

    await exchangeRouter.connect(user0).multicall([
      exchangeRouter.interface.encodeFunctionData("sendTokens", [usdc.address, externalHandler.address, amount]),
      exchangeRouter.interface.encodeFunctionData("makeExternalCalls", [[], [], [usdc.address], [user0.address]]),
    ]);

    expect(await usdc.balanceOf(externalHandler.address)).eq(0);
    expect(await usdc.balanceOf(user0.address)).eq(before);
  });

  it("restores ExternalHandler state when a later multicall operation reverts", async () => {
    const amount = expandDecimals(25, 6);
    await usdc.mint(user0.address, amount);
    await usdc.connect(user0).approve(router.address, amount);

    await expect(
      exchangeRouter.connect(user0).multicall([
        exchangeRouter.interface.encodeFunctionData("sendTokens", [usdc.address, externalHandler.address, amount]),
        exchangeRouter.interface.encodeFunctionData("makeExternalCalls", [[], [], [], []]),
        exchangeRouter.interface.encodeFunctionData("sendTokens", [usdc.address, orderVault.address, amount.mul(2)]),
      ])
    ).to.be.reverted;

    expect(await usdc.balanceOf(externalHandler.address)).eq(0);
  });
});
