# GMX — Cross-Architecture DeFi Security Research: EVM, Solana, Economic Invariants & Evidence-Driven Validation

## Executive Summary

This is independent security research across selected GMX Synthetics EVM/Solidity, GMX Solana Rust/Anchor, and GMX V1/Solidity surfaces. It is not an official GMX audit, client engagement, certification, or claim of protocol acceptance.

The campaign’s value is its reasoning chain: value-flow reconstruction, trust-boundary analysis, explicit security properties, targeted experiments, counter-tests, and conservative economic qualification. Thirty-six historical investigation records were retained, but they are not thirty-six vulnerabilities. They include duplicates, expected behavior, model-only observations, fixture-limited behavior, source-limited hypotheses and unresolved questions.

## Research Scope and Boundaries

The EVM work examined selected Synthetics mechanisms: subaccounts, integrations, signed approvals, relays, deadlines, external-handler paths, vault balances and interactions between reader logic and state-changing paths. The Solana work focused on Rust model and Anchor-oriented paths involving market state, positions, borrowing/funding clocks and temporal transitions. GMX V1 remained source-limited.

These were targeted investigations, not complete reviews of every deployment, market, integration or configuration. Compiled-file counts are build metrics, not counts of contracts manually audited. Test counts are execution evidence, not vulnerability counts.

## Architecture and Threat Modeling

The threat model began by identifying actors and trust boundaries: position owners, authorized subaccounts, integrations, keepers, vaults, external handlers and configurable administrative roles. For each flow, the question was which authority should authorize a transition and which component ultimately moves value.

On EVM, this meant following signed data, nonces, deadlines and integration state across router and subaccount boundaries. It also meant separating an authorization failure from a fixture that preconfigured a privileged role. On Solana, oracle-reported conditions were kept separate from persisted market flags, and model behavior was kept separate from deployed-program behavior.

## Value Flow and Economic Invariants

Value-flow reconstruction identified assets, holders, accounting variables and state-changing modules before a hypothesis was tested. Candidate invariants covered physical versus accounted balances, fee and collateral conservation, position lifecycle transitions, authorization boundaries and temporal accrual.

An invariant had to be expressed as a falsifiable assertion and then challenged. A failing relation was not automatically a finding: the comparison might involve mutable state, rounding, configuration or an invalid economic assumption. This discipline was particularly important for model-based position and fee experiments.

## EVM Security Research Methodology

The EVM investigations combined Solidity source reading, TypeScript/Hardhat tests, fixture inspection and clean-build verification:

1. Identify the entry point and caller roles.
2. Trace calls that change permissions, balances or accounting state.
3. Define positive and negative controls.
4. Record the exact assertion reached.
5. Test alternative explanations such as mocks, missing funds, role setup, ABI errors and expected reverts.
6. Classify the result as production behavior, fixture behavior, model behavior or blocked.

The Relay work illustrates why source history and design intent matter. Historical local tests reported behavior around disabled integrations, while later review supported a separate association model. The appropriate public conclusion is a specification and integration analysis, not a confirmed production bypass.

External-handler and OrderVault research examined residual-balance accounting. Clean-build tests reproduced local assertions, but the provenance of residual balances and their relationship to an official production flow remained unproven. A preloaded balance is not, by itself, evidence of theft.

## Solana/Rust Model-Based Research

Rust model tests examined market state, position lifecycle and accounting clocks. One targeted model test freshly reproduced a bounded borrowing-timing difference under a stale-state precondition. The corresponding Anchor source contains fixture assertions for clock advancement, relative accrual and non-recovery after a later state update.

The model result is classified as `FRESH_MODEL_PASS`, not as a deployed-program exploit. Anchor execution was not freshly reproduced in the final validation because the required CLI and historical validator environment were unavailable; it remains `HISTORICAL_ANCHOR_ONLY` / `BLOCKED_ENVIRONMENT`. A reproducible model can reveal a coherent accounting path without proving production equivalence.

## Testing Infrastructure and Reproducibility

Evidence engineering was part of the work. Isolated copies, recorded toolchains, clean output directories, build metadata, source hashes and raw transcripts were used where available.

A clean isolated Hardhat build compiled 431 Solidity files and then executed 15 targeted tests against newly generated artifacts: four Relay, seven ExternalHandler and four OrderVault tests. These tests validate local fixture assertions only.

The Rust validation recorded the toolchain, copied the modified research checkout to an isolated workspace and retained the raw Cargo transcript. Missing Anchor/Solana tools were recorded as a blocker rather than converted into a pass.

## Evidence Qualification and Counter-Testing

Historical reports, fresh execution and blocked reproduction were kept separate. Evidence classes included `EXPECTED_BEHAVIOR`, `LOCAL_FIXTURE_ONLY`, `MODEL_LIMITED`, `SOURCE_LIMITED` and `HYPOTHESIS_UNVERIFIED`.

Counter-tests searched for legitimate permissions, artificial role setup, preloaded balances, mocked feeds, stale configuration, rounding, mutable state and missing economic provenance. This prevented a report title or a green test from outranking the actual assertion and preconditions.

## False-Positive Elimination

Several hypotheses were downgraded rather than promoted. Some apparent authorization issues were consistent with documented association flows. Some residual-balance observations depended on fixture state. Some model relations failed without proving that the proposed invariant was valid. Some records lacked source or transcript evidence and remained unverified.

This is a core audit skill: a security investigation must be able to close a plausible hypothesis when the evidence does not support it.

## How the Reasoning Was Applied

The investigations were organized as small, reviewable questions rather than as a single undifferentiated search. For an authorization question, the analysis followed the signed intent, the caller, the stored association and the final state change. For an accounting question, it compared the source of a balance with the operation that consumed it. For a temporal question, it recorded which clock advanced, which state flag was persisted and whether a later transition could recover an interval already consumed.

This structure made negative results useful. A revert caused by an invalid path, missing funds or an unactivated feature was not described as proof of a security control. A passing test whose assertion depended on a preloaded balance was retained as a fixture observation, not as proof that production users could create or capture that balance. A model failure was reviewed as a possible specification error, rounding effect or mutable-state artifact before it was considered a protocol anomaly.

The process also required explicit provenance. Source revisions, modified working-tree state, compiler versions, build-info and transcript hashes were recorded when available. Where a historical result could not be independently reproduced, it remained historical. Where an environment was unavailable, it remained blocked. This distinction is important for professional review because reproducibility is not binary: a clean local build, a model test, a fork test and a deployed-program test answer different questions.

## What a Reviewer Can Inspect

An experienced reviewer can use the public case study as a map to the underlying work. The EVM portion demonstrates how a researcher can move from a router or vault entry point to roles, balances, assertions and counter-tests. The Solana portion demonstrates how to state a model result without confusing it with Anchor execution. The evidence section explains why the same green test result has different weight depending on its source, fixture and artifact provenance.

The campaign also illustrates an important audit habit: preserve the failed experiment. The partial-close model relation was not rewritten after an assertion failed. Instead, the failure remained an unresolved question because the one-shot and split sequences mutated state differently. Preserving that uncertainty is more informative than changing the tolerance until the test passes.

## Reusable Engagement Pattern

For a private engagement, this workflow can be reused on a defined module: map assets and actors, identify the trust boundary, write the invariant, construct a minimal fixture, add positive and negative controls, inspect the resulting state transition, and qualify the economic effect. The deliverable should state the exact scope and assumptions, include reproducible commands where authorized, and distinguish a confirmed issue from a lead requiring protocol-owner clarification.

The campaign therefore demonstrates a practical research pattern rather than a promise of universal coverage. It is suitable for targeted reviews of integrations, accounting modules, fee paths, state transitions and property-test suites. It is not evidence that every GMX market, deployment or configuration has been audited.

## Selected Research Metrics

- 36 historical investigation records, not 36 vulnerabilities.
- Three GMX architectures represented.
- 431 Solidity files compiled in clean-build validation.
- 15 targeted local Hardhat tests passed after compilation.
- Four Relay, seven ExternalHandler and four OrderVault tests.
- One fresh Rust model Borrowing test.
- No confirmed permissionless GMX economic finding.

## Disclosure Boundaries

This public study intentionally omits private PoCs, exact payloads, sensitive preconditions, addresses, local paths, raw forensic material and unpublished economic scenarios. Existing public history is not treated as blanket authorization to amplify technical details. Any future disclosure requires human review.

## Professional Research Takeaways

The work demonstrates architecture reconstruction, trust-boundary analysis, invariant design, TypeScript/Hardhat testing, Rust model testing, source/build provenance, counter-testing and economic qualification. The reusable professional capability is targeted DeFi investigation: reconstruct a value flow, formulate a property, build a bounded experiment, challenge it and report only what the evidence supports.

## Limitations and Independent Research Status

This is independent research, not a client audit. Production deployments, all configurations and all historical environments were not exhaustively validated. Anchor reproduction remains blocked; several findings remain private or source-limited. No GMX security certification, official validation or confirmed production exploit is claimed.
