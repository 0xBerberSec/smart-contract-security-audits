# GMX — Offensive DeFi Security Research & Reproducible Testing

**EVM accounting | Hardhat/TypeScript PoC engineering | Rust economic invariants | cross-architecture attack-surface analysis**

This is the executable code companion to the [GMX cross-architecture case
study](../gmx-cross-architecture-security-research.md). It shows how selected
security hypotheses were reduced to assertions, counter-controls and
reproducible local experiments.

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

## Scope and limitations

This is independent research and a reproducible local harness, not an official
GMX audit or certification. It uses no production RPC, transactions or real
funds. The EVM test is a sanitized derivation; the Rust result is model-only;
Anchor execution is not claimed. A failing invariant is a research result to
investigate, not a confirmed vulnerability.
