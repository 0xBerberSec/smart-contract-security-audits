# GMX portfolio cleanup result

## Scope

Prepared on branch `gmx-professional-portfolio-cleanup` from
`origin/main` `6d52dd9fecdcae42fa78e5a55903c47bff229271`. No Solidity, TypeScript,
Rust model, transcript, lockfile or licence file was deleted or rewritten.
The legacy Relay directory remains an explicitly labelled compatibility archive.

## Changes

- Reordered the central harness README so the real GMX adapted TypeScript tests
  and the one-command clean-checkout runner appear before `RefundLedger.sol`.
- Added direct navigation to ExternalHandler/ExchangeRouter and
  OrderVault/DepositVault tests, the Borrowing and Partial Close model tests,
  and the evidence index.
- Added `evidence/README.md`, separating `CURRENT_VERIFIED` results from
  `HISTORICAL_OR_BLOCKED` traces without modifying the raw logs.
- Marked `evm/README.md` as the secondary educational laboratory and linked it
  back to the real GMX tests.
- Added the canonical runner link to `evm/original-gmx-tests/README.md`.

No small README was deleted: each retained file carries a distinct source,
scope, licensing or control-path reference, and all existing links remain valid.

## Metrics preserved

- RefundLedger educational harness: 3 PASS.
- Public GMX adaptations: 4 PASS.
- Private original GMX suite: 11 PASS.
- Historical targeted total: 15 PASS.
- Historical isolated clean-build: 431 Solidity files.
- New clean-checkout reproduction: 430 Solidity files.
- Rust Borrowing: `FRESH_MODEL_PASS`, exit 0.
- Rust Partial Close: `FRESH_FAIL`, exit 101.
- Anchor: `NOT_REPRODUCED`.

These are separate evidence classes and are not added together as independent
campaigns.

## Validation

The clean-checkout runner, EVM educational tests and Rust model commands were
already executed and their statuses are preserved. Markdown relative links in
the changed GMX scope resolve locally; `git diff --check` is required before
commit. No production RPC, transaction, private key or original private test is
introduced.

## Recruiter path

30 seconds: the parent README exposes the real GMX tests and command first.
Three minutes: links lead directly to assertions, model tests and evidence
classification. Twenty minutes: the reviewer can follow
question → surface → invariant → test → assertion → result → limitation, while
the private original suite and Anchor boundary remain explicit.
