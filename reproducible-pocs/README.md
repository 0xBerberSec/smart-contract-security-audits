# Reproducible PoCs

**No runnable protocol PoC is included in this public selection.** Existing research code and historical execution artifacts remain local while source provenance and disclosure rights are reviewed. This page does not claim a publicly reproducible technical finding.

A future released reproduction should include:

- The exact source revision and test patch, with relevant dependency licenses.
- A pinned toolchain and dependency lockfile, and one documented command.
- Explicit actors, privileges, mocks and fixture mutations.
- The chain, fork block and RPC requirements when applicable.
- The assertion, baseline, control case, raw result and economic limitations.
- Separate labels for historical results and fresh executions.

A passing test may confirm expected behavior, a counter-test or an expected panic. A failed setup is not an invariant violation. [Testing approach](../research-tools-testing/testing.md) · [Evidence register](../docs/EVIDENCE.md).

[Portfolio](../README.md)
