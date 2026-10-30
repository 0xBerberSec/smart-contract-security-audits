# Verification and Execution Status

## Educational experiment — 30 October 2026

The [Refund Escrow execution record](../educational-demos/refund-escrow/EVIDENCE.md) links the real source, commands, outputs and input hashes. Forge 1.7.1 and Solc 0.8.24 executed the default suite: **13 passed, 0 failed, 0 skipped**. Two fuzz tests used 256 cases each. The fixed-contract invariant ran 128 sequences of depth 64: 8,192 handler calls and zero handler-level reverts.

A separate profile executes a safety property against the deliberately vulnerable version. It returned **exit code 1**, with one failed test at the intended `INVARIANT_BROKEN` assertion. This expected failure is retained with its trace, outside the default suite; it is not silently counted as a pass.

The example is a newly prepared educational contract using a known vulnerability class. It does not add a third-party finding, platform validation or completed client audit. Handler scope and economic limits are documented. Private protocol execution records are outside this public evidence set.

## Reproduce and review

- [Local execution guide](../educational-demos/refund-escrow/README.md)
- [Recorded evidence](../educational-demos/refund-escrow/evidence/review-2026-10-30/RUN.json)
- [Testing and interpretation](../research-tools-testing/testing.md)

Repository link, privacy and Git whitespace checks validate the documentation package, not protocol security.

[Portfolio](../README.md)
