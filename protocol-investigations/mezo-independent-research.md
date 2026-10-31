# Mezo — Independent Protocol Security Research

This section documents independent research performed against Mezo using an Audit-OS workflow. It is not a client audit, security certification, or externally validated vulnerability report.

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

I used Audit-OS to organize value flows, assumptions, hypotheses, tests and counter-tests. Codex assisted source inspection, harness preparation, execution and documentation. The published conclusions are bounded by the assertions and execution records described in each study. The Pool fuzzer is a fixed-state quote experiment; Router PASS labels record low-level rejection without proving its internal cause.

Only these selected methodological summaries are public. Execution records and source hashes are retained privately, and the [runnable educational lab](../educational-demos/refund-escrow/README.md) provides a separate public example of the method.
