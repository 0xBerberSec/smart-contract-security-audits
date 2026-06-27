# Moonwell Comptroller - BorrowAllowed Market Entry Persistence

[Back to portfolio](../../README.md)

## Type

Research Report / Proof of Concept

## Status

Public research. This is not presented as a validated contest finding.

## Scope

- Protocol: Moonwell
- Network: Base
- Contract: `Comptroller.sol`
- Function analyzed: `borrowAllowed(address mToken, address borrower, uint borrowAmount)`

## Summary

During fork-based analysis of Moonwell's Comptroller contract on Base, persistent state mutation behavior was observed in `borrowAllowed()`.

The function can add a borrower to a market before validating whether the borrow is permitted. If the borrow fails, the borrower may remain entered in the market.

## Technical Explanation

Inside `borrowAllowed()`:

1. `addToMarketInternal()` mutates `accountMembership`
2. Liquidity validation occurs afterward via `getHypotheticalAccountLiquidityInternal`
3. If validation fails, the function returns an error
4. The state mutation is not reverted

This results in persistent market membership despite an unsuccessful borrow attempt.

## Technical Focus

- Protocol invariant review
- Compound-style lending logic
- State mutation before final authorization
- Fork-based reproduction

## Proof of Concept

The PoC demonstrates:

- Initial state: borrower is not entered in any market
- Borrow attempt fails
- Market membership persists afterward
- `getAssetsIn()` reflects the new market entry

The PoC test is included in this directory:

- [`BorrowAllowedDirtyStateFork.t.sol`](./BorrowAllowedDirtyStateFork.t.sol)

## Environment

- Network: Base mainnet fork
- Tooling: Foundry
- Language: Solidity

## Reproduction

```bash
forge test --fork-url <BASE_RPC> --match-contract BorrowAllowedDirtyStateFork
```

## Discussion

This behavior highlights a non-atomic state mutation occurring before final borrow validation.

While this behavior may align with certain Compound-style design assumptions, it demonstrates how policy-check functions can introduce persistent state changes before final authorization logic completes.

This analysis focuses on protocol invariants, state consistency, and fork-based validation methodology.

## Conclusion

This report highlights a non-atomic state mutation pattern and documents the behavior through a fork-based PoC.

[Back to portfolio](../../README.md)
