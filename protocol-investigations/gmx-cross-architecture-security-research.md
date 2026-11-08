# GMX — Cross-Architecture DeFi Security Research

## Scope

I investigated selected GMX Synthetics EVM/Solidity, GMX Solana Rust/Anchor and
GMX V1/Solidity surfaces. This is independent research, not a client audit,
certification or claim of GMX acceptance.

The accompanying [reproduction harness](gmx-reproduction/README.md) contains
the public EVM adaptations, the source-preserving Rust model and the execution
evidence. Private findings, payloads and sensitive fixture sequences remain
outside the repository.

The archive contains 36 investigation records, not 36 vulnerabilities. It
includes expected behavior, duplicate hypotheses, fixture-limited observations,
model-only results, source-limited questions and unresolved items.

## Research questions and surfaces

On EVM I followed subaccounts, integrations, signed approvals, relays,
deadlines, external-handler calls and vault balances across router and state
changing paths. I treated users, authorized subaccounts, integrations, keepers,
vaults, external handlers and configurable roles as separate trust boundaries.

On Solana I examined market state, position lifecycle, borrowing/funding clocks
and temporal transitions in the Rust model, then compared the model assumptions
with Anchor-oriented source paths. GMX V1 remained source-limited.

These were targeted investigations, not complete reviews of every deployment,
market, integration or configuration. File counts are build metrics; test counts
are execution records, not vulnerability counts.

## Method

For each question I used the following sequence:

1. Trace assets, actors and the state-changing entry point.
2. Identify the authorization or accounting boundary.
3. State a falsifiable invariant.
4. Build a fixture with explicit positive and negative controls.
5. Execute the assertion and record the resulting state.
6. Test competing explanations: roles, mocks, preloaded balances, reverts,
   rounding, mutable state and configuration.
7. Separate production behavior from fixture, model and blocked evidence.

I closed or reclassified hypotheses when the evidence did not support them. A
passing assertion did not override missing production preconditions; a failed
model relation was preserved until its assumptions could be examined.

## EVM investigations

### Relay and integration boundaries

I traced signed approvals, nonces, deadlines and integration state through the
router/subaccount boundary. Local tests and source history were used to
distinguish a stored association from an identifier carried in signed data. The
public conclusion is limited to a design/specification analysis; no
permissionless production bypass is claimed.

### ExternalHandler and refund accounting

The relevant path crosses `ExchangeRouter`, `ExternalHandler` and
`PayableMulticall`. The public adaptation
[`external-handler-safe-controls.spec.ts`](gmx-reproduction/evm/original-gmx-tests/external-handler-safe-controls.spec.ts)
checks two controls: an explicitly listed refund token is returned and a later
multicall revert rolls back the earlier transfer. The private seven-test suite
also examined residual and multi-token paths; those exact sequences remain
private.

The tests establish fixture-level balance and rollback properties. They do not
establish that an untrusted production caller can create the same residual
state, that a balance belongs to another user, or that funds can be extracted.

### OrderVault and internal accounting

I compared physical ERC-20 balances with the vault's internal `tokenBalances`
accounting across OrderVault and DepositVault flows. The public
[`vault-accounting-observations.spec.ts`](gmx-reproduction/evm/original-gmx-tests/vault-accounting-observations.spec.ts)
keeps the non-sensitive balance and unrelated-user controls. The private suite
contains the complete increase, deposit, GLV and withdrawal continuations.

The observation is local to a deliberately pre-funded fixture. It does not
prove token provenance, third-party ownership or production loss.

## Solana model experiments

### Borrowing

The model test
`stale_regular_borrowing_update_consumes_closed_duration` compares a stale
regular borrowing update with the closed-market factor over the same elapsed
interval. It passes under Rust 1.89 (`FRESH_MODEL_PASS`, exit 0). The output
records the factors, duration and missed fee calculation. This is a model result,
not Anchor or deployed-program evidence.

### Partial Close

`partial_close_cannot_extract_more_pnl_than_full_close` compares one full close
with split closes. The assertion fails under Rust 1.89 (`FRESH_FAIL`, exit 101)
with concrete PnL and output-value differences. I retained the failure instead
of changing the tolerance. It falsifies the proposed model relation for the
fixture; it does not confirm a Solana vulnerability or production loss.

Anchor execution remains `NOT_REPRODUCED`; the required validator environment
was not part of the final public reproduction.

## Reproduction and evidence

The clean-checkout runner is
[`reproduce-original-evm.py`](gmx-reproduction/scripts/reproduce-original-evm.py).
It clones GMX Synthetics at commit
`10cfbce6144ade8e9d5cd8c811573955bcd1c132`, installs its lockfile, compiles and
runs the two public adaptations. The verified result is four PASS tests, exit 0.

The historical isolated build compiled 431 Solidity files and ran 15 targeted
Hardhat tests (4 Relay, 7 ExternalHandler, 4 OrderVault). The newer clean
checkout compiled 430 files. These are distinct executions/configurations; no
cause is inferred for the difference.

The public harness also contains three educational RefundLedger tests (3 PASS),
the private original EVM result (11 PASS), the Borrowing model PASS, the Partial
Close model FAIL and the Anchor `NOT_REPRODUCED` status. They are separate
evidence classes and are not added together as one campaign.

## Conclusions and limits

The experiments establish how the selected fixtures, models and controls
behave. They do not establish complete GMX coverage, production exploitability,
official validation or user loss. Source revisions, lockfiles, licences,
transcripts and current/historical classifications are indexed in the
[harness evidence directory](gmx-reproduction/evidence/README.md).
