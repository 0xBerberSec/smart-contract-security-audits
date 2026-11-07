# GMX original EVM investigations

Start with the repository-level [clean-checkout runner](../../scripts/reproduce-original-evm.py)
and [reproduction instructions](REPRODUCTION.md). It is the canonical path for
the four public adapted tests.

This directory contains two small, reviewable TypeScript adaptations of the
original GMX Synthetics EVM investigations. The private reproduction executed
all eleven original Hardhat functions freshly: seven ExternalHandler tests and
four OrderVault tests, all PASS with exit 0.

The exact original files remain private because they include research-specific
residual-balance and external-call sequences. The two public files retain the
real GMX fixture, contract calls and assertions for non-sensitive controls:

- [`external-handler-safe-controls.spec.ts`](./external-handler-safe-controls.spec.ts)
- [`vault-accounting-observations.spec.ts`](./vault-accounting-observations.spec.ts)

They are marked `PUBLIC ADAPTATION` in source. The original files, fixtures,
mocks, artifact hashes and raw transcripts remain in the private package.

## Scope

- ExternalHandler: refund coverage, partial-fill leftovers, multi-token output
  handling and rollback controls.
- OrderVault: physical ERC-20 balances versus internal `tokenBalances` across
  increase, deposit, GLV deposit and withdrawal flows.

These are local fixture experiments. A PASS confirms the assertions in the
fixture; it does not establish a permissionless production exploit or an
externally validated GMX finding.

## Reproduction

Place each adapted file under `test/guardian/` in the matching GMX Synthetics
checkout at revision `10cfbce6144ade8e9d5cd8c811573955bcd1c132`, then run:

```text
npx hardhat test --no-compile test/guardian/external-handler-safe-controls.spec.ts
npx hardhat test --no-compile test/guardian/vault-accounting-observations.spec.ts
```

Fresh isolated results for these adaptations were 2 passing / exit 0 each.

## Private evidence package

The exact source and transcript package remains private pending disclosure
approval. The existing public `RefundLedger.sol` remains a separate sanitized
derivation and is not presented as the original GMX test suite.
