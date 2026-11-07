# GMX — Offensive DeFi Security Research & Reproducible Testing

**EVM accounting | Hardhat/TypeScript PoC engineering | Rust economic invariants | cross-architecture attack-surface analysis**

This is the executable code companion to the [GMX cross-architecture case
study](../gmx-cross-architecture-security-research.md). It shows how selected
security hypotheses were reduced to assertions, counter-controls and
reproducible local experiments.

## Start with the real GMX tests

### GMX Synthetics EVM — real-contract security tests

These public adaptations execute against the actual GMX Synthetics fixture and
contracts. They are the fastest way to inspect the protocol-facing work:

- **ExternalHandler / ExchangeRouter:** [external-handler-safe-controls.spec.ts](evm/original-gmx-tests/external-handler-safe-controls.spec.ts) — explicit refund, multicall rollback and post-revert balance assertions.
- **OrderVault / DepositVault:** [vault-accounting-observations.spec.ts](evm/original-gmx-tests/vault-accounting-observations.spec.ts) — physical ERC-20 balance versus `tokenBalances` and unrelated-user controls.
- **One-command clean checkout:** [reproduce-original-evm.py](scripts/reproduce-original-evm.py).

Run from the repository root:

```text
python protocol-investigations/gmx-reproduction/scripts/reproduce-original-evm.py
```

The runner clones GMX Synthetics, pins commit
`10cfbce6144ade8e9d5cd8c811573955bcd1c132`, installs its lockfile, compiles,
copies the two public adaptations and executes four assertions. A PASS is
printed only after those commands actually complete. The latest clean-checkout
result is **4 PASS, exit 0**.

### Rust model experiments

- **Borrowing:** [source test](solana/model/src/test.rs) —
  `stale_regular_borrowing_update_consumes_closed_duration`, **FRESH_MODEL_PASS,
  exit 0**.
- **Partial Close:** [source property](solana/model/src/action/decrease_position/mod.rs) —
  `partial_close_cannot_extract_more_pnl_than_full_close`, **FRESH_FAIL, exit
  101**. This falsifies a proposed model invariant; it is not a confirmed
  production vulnerability.

For the exact evidence classification and historical/current split, see the
[evidence index](evidence/README.md).

## Research overview

The private campaign covered selected GMX Synthetics EVM, GMX Solana and GMX V1
surfaces. The public harness deliberately exposes only two safe evidence
classes:

- **EVM — `SANITIZED_DERIVATION`**: a small Hardhat contract and TypeScript test
  derived from the accounting questions in the private ExternalHandler work.
- **Rust — source-preserving model test**: the GMX Solana model source and its
  original Partial Close property test, with the source licence retained.

The EVM code is not the original GMX contract and is not a production exploit
reproduction. The Rust test is a model-level experiment, not an Anchor or
deployed-program result.

## Technical capabilities demonstrated

- Translate an asset-accounting hypothesis into explicit state assertions.
- Pair a positive control with an omitted-asset negative control.
- Assert rollback after a later route failure.
- Trace a Rust financial state machine through full and partial position closes.
- Preserve a failing invariant as evidence instead of changing it to obtain a
  passing result.
- Record toolchain, dependency, lockfile and licence provenance.

## Case study A — EVM accounting and refund attack-surface testing

Inspect [RefundLedger.sol](evm/contracts/RefundLedger.sol) and
[refund-ledger.spec.ts](evm/test/refund-ledger.spec.ts). The three tests check:

1. an explicitly listed balance is fully refunded;
2. an omitted balance remains accounted for;
3. a later revert rolls back the route state.

These are the same classes of accounting questions used during the private
ExternalHandler investigation, but the public contract is a sanitized,
stand-alone derivation. It contains no GMX addresses, deployment assumptions,
relay payloads or private finding conditions.

## Case study B — Rust economic-invariant falsification

The [Partial Close test](solana/model/src/action/decrease_position/mod.rs)
compares one full close with sequences of partial closes across multiple PnL
factors and position directions. Under Rust 1.89 it reaches the original
assertion and returns:

```text
FRESH_FAIL — exit code 101
```

The result means the proposed model invariant was falsified for at least one
fixture. It does **not** prove a Solana program vulnerability, a permissionless
exploit or an economic loss in production. The exact metrics are preserved in
the execution evidence; the model and its assumptions require further
technical interpretation.

## Case study C — cross-architecture research

The broader study connects Solidity/EVM authorization and accounting work with
Rust/Solana state-transition analysis. Historical GMX results are intentionally
separated from this public harness:

- the earlier **15 Hardhat PASS** results were private clean-build validation
  runs, not the three public EVM tests here;
- Anchor was not freshly reproduced;
- the 36 historical records are investigation records, not 36 vulnerabilities;
- no externally confirmed permissionless GMX finding is claimed.

## Attack surface and trust boundaries

The underlying research examined boundaries between users, subaccounts,
integrations, relays, external handlers and vault accounting. This public
package demonstrates the method without exposing the sensitive Relay test
sequence. The useful review question is whether the accounting property still
holds when a listed asset, omitted asset or reverted route changes the state.

## Run the experiments

### EVM

```text
cd protocol-investigations/gmx-reproduction/evm
npm install --ignore-scripts --no-audit --no-fund
npx hardhat test test/refund-ledger.spec.ts --show-stack-traces
```

Expected recorded result: **3 passing, exit code 0**.

### Rust

```text
cd protocol-investigations/gmx-reproduction/solana/model
rustup run 1.89 rustc --version
rustup run 1.89 cargo --version
cargo +1.89 test --locked partial_close_cannot_extract_more_pnl_than_full_close -- --nocapture
```

Expected recorded result for this snapshot: **FRESH_FAIL, exit code 101**.

The separately named Borrowing model test is:

```text
cargo +1.89 test --locked stale_regular_borrowing_update_consumes_closed_duration -- --nocapture
```

It is a different model assertion from Partial Close. Its status must be read
from the command output; no Borrowing result is inferred from the Partial Close
transcript.

The raw, path-redacted logs and exit codes are in [evidence](evidence/). The
public Cargo manifest pins the source-compatible dependency versions; the
original lock snapshot and the regenerated public lock are both hashed.

## Source-code navigation

- [Original EVM investigation index](evm/original-gmx-tests/README.md)
- [ExternalHandler adapted controls](evm/original-gmx-tests/external-handler-safe-controls.spec.ts)
- [OrderVault accounting observations](evm/original-gmx-tests/vault-accounting-observations.spec.ts)
- [EVM contract](evm/contracts/RefundLedger.sol)
- [EVM assertions](evm/test/refund-ledger.spec.ts)
- [Rust Partial Close property](solana/model/src/action/decrease_position/mod.rs)
- [Rust model crate](solana/model/src/)
- [Execution evidence](evidence/)
- [Provenance and licence notices](THIRD_PARTY_NOTICES.md)

## Evidence status

| Investigation | Type of evidence | Public status |
|---|---|---|
| EVM RefundLedger | Educational derivation | 3 PASS |
| GMX ExternalHandler / OrderVault | Adapted tests on real GMX components | Clean-checkout runner; historical snapshot 4 PASS |
| GMX original tests | Private original functions and transcripts | 11 PASS, not shipped as executable public tests |
| Rust Borrowing | Source-preserving model test | `FRESH_MODEL_PASS`, 1 PASS, exit 0 |
| Rust Partial Close | Source-preserving model invariant | `FRESH_FAIL`, exit 101 |
| Anchor | Program/fixture execution | `NOT_REPRODUCED` |

Borrowing and Partial Close are different tests and must not be combined. A
model PASS is not an Anchor or production result; the Partial Close failure is a
counterexample to the proposed model relation, not a confirmed GMX exploit.

## Scope and limitations

This is independent research and a reproducible local harness, not an official
GMX audit or certification. It uses no production RPC, transactions or real
funds. The EVM test is a sanitized derivation; the Rust result is model-only;
Anchor execution is not claimed. A failing invariant is a research result to
investigate, not a confirmed vulnerability.
