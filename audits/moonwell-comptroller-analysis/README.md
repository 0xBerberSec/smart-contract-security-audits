# Moonwell – Comptroller Behavioral Analysis

## Scope

- Protocol: Moonwell
- Network: Base
- Contract: Comptroller.sol
- Function analyzed: borrowAllowed(address mToken, address borrower, uint borrowAmount)

---

## Overview

During a fork-based analysis of Moonwell’s Comptroller contract on Base, a persistent state mutation behavior was observed in `borrowAllowed()`.

The function adds a borrower to a market before validating whether the borrow is actually permitted.

If the borrow fails (e.g. insufficient liquidity or collateral), the borrower remains permanently entered in the market.

---

## Technical Explanation

Inside `borrowAllowed()`:

1. `addToMarketInternal()` mutates `accountMembership`
2. Liquidity validation occurs afterward via `getHypotheticalAccountLiquidityInternal`
3. If validation fails, the function returns an error
4. The state mutation is not reverted

This results in persistent market membership despite an unsuccessful borrow attempt.

---

## Proof of Concept

The behavior was reproduced on a Base mainnet fork using Foundry.

The test demonstrates:

- Initial state: borrower not in any market
- Borrow attempt fails
- Market membership persists
- `getAssetsIn()` reflects the new market entry

The PoC test is included in this directory.

---

## Discussion

This behavior highlights a non-atomic state mutation occurring prior to final borrow validation.

While such behavior may align with certain Compound-style design assumptions, it demonstrates how policy-check functions can introduce persistent state changes before final authorization logic completes.

This analysis focuses on protocol invariants, state consistency, and fork-based validation methodology.

---

## Tools Used

- Foundry (fork testing)
- Base mainnet fork
- Manual line-by-line review
- Transaction trace inspection

---

## Conclusion

This research demonstrates practical fork-based behavioral analysis of core DeFi lending logic and invariant validation in production contracts.

## Reproducibility

Tested on Base mainnet fork using Foundry.

forge test --fork-url <BASE_RPC> --match-contract BorrowAllowedDirtyStateFork


