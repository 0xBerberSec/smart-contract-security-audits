# Mezo — Independent Protocol Security Research

This section documents independent research performed against Mezo using an Audit-OS workflow. It is not a client audit, security certification, or externally validated vulnerability report.

## Current result

The published experiments establish bounded local properties on a Mezo fork at
block `12674080` (chain ID `31612`). Pool quote checks covered 60 valid grid
calls and 1,024 material-input fuzz cases. Router controls reproduced exact
negative paths under a Cancun-compatible environment; the earlier London PASS
labels were traced to `NotActivated`. No permissionless economic exploit was
demonstrated. The detailed studies below retain the original assertions and
their limits.

## What was studied

- AMM Pool reserve/quote accounting and bounded fuzz properties.
- UniversalRouter command, deadline, allowance and residual-balance boundaries.

## Evidence policy

Historical transcripts, source review, local fixture tests and current reproductions are separate evidence classes. The selected Pool and Router harnesses were reproduced in an isolated local fork; the results demonstrate bounded local properties and negative boundary checks, not a permissionless economic exploit.

## Case studies

- [Pool accounting and bounded fuzzing](mezo-pool-fuzz.md)
- [UniversalRouter permissions and residuals](mezo-universal-router.md)

Sensitive replay details, raw signatures, unresolved bridge hypotheses and private RPC material are intentionally withheld.

## Research workflow

I used Audit-OS to organize value flows, assumptions, hypotheses, tests and counter-tests. Codex assisted source inspection, harness preparation, execution and documentation. The published conclusions are bounded by the assertions and execution records described in each study. The Pool fuzzer is a fixed-state quote experiment; historical Router PASS labels record low-level rejection; the UniversalRouter London results were subsequently traced to `NotActivated` and are not functional-control evidence.

## Public material and private boundaries

The public selection comprises these Pool and UniversalRouter studies, the [five selected harness sources, Foundry configuration and runner](mezo-reproduction/README.md), and a [sanitized reference record](mezo-reproduction/evidence/reference-run.json) with selected results and source hashes. The runner supports local replay subject to the recorded tools and a compatible historical Mezo RPC. No hosted Mezo CI is claimed.

Raw transcripts not selected for release, unrelated private protocol investigations, non-disclosable vulnerability hypotheses, internal review records and other evidence outside this harness remain private. These categories identify the disclosure boundary; no technical details from those investigations are included here.

Public availability of the selected sources does not imply a completed client audit, external validation or confirmed exploit. The [runnable educational lab](../educational-demos/refund-escrow/README.md) remains a separate synthetic demonstration of the method.

The [dated evidence correction and separate complementary tests](mezo-reproduction/supplemental/README.md) add measured quote coverage, positive controls and exact errors. They do not overwrite historical results, establish a full stateful campaign or claim an exploitable finding.
