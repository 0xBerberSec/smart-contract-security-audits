# Verification and Execution Status

## Educational experiment — 30 October 2026

The [Refund Escrow execution record](../educational-demos/refund-escrow/EVIDENCE.md) links the real source, commands, outputs and input hashes. Forge 1.7.1 and Solc 0.8.24 executed the default suite: **13 passed, 0 failed, 0 skipped**. Two fuzz tests used 256 cases each. The fixed-contract invariant ran 128 sequences of depth 64: 8,192 handler calls and zero handler-level reverts.

A separate profile executes a safety property against the deliberately vulnerable version. It returned **exit code 1**, with one failed test at the intended `INVARIANT_BROKEN` assertion. This expected failure is retained with its trace, outside the default suite; it is not silently counted as a pass.

The example is a newly prepared educational contract using a known vulnerability class. It does not add a third-party finding, platform validation or completed client audit. Handler scope and economic limits are documented. Private protocol execution records are outside this public evidence set.

## Reproduce and review

The links in this subsection refer to the educational experiment above. Mezo has a separate harness below.

- [Local execution guide](../educational-demos/refund-escrow/README.md)
- [Recorded evidence](../educational-demos/refund-escrow/evidence/review-2026-10-30/RUN.json)
- [Testing and interpretation](../research-tools-testing/testing.md)

Repository link, privacy and Git whitespace checks validate the documentation package, not protocol security.

## Mezo protocol research — 31 October 2026

The selected historical harnesses were reproduced on a local Mezo fork at block **12674080**, chain ID **31612**, using Forge **1.7.1** (`4072e48705af9d93e3c0f6e29e93b5e9a40caed8`), Solidity **0.8.24** and **London**. No public transaction was sent.

| Suite | Recorded result | What the assertion checks |
| --- | --- | --- |
| `MezoPoolBoundedStatefulFuzz` | 1 PASS, 1,024 cases | Successful quote output does not exceed the selected reserve; zero-input conditional check |
| `MezoPoolAccountingInvariants` | 2 PASS | Balance/reserve coverage, LP supply floor, reserve-view agreement, deterministic zero-input quotes |
| `MezoRouterSlippageAndDeadline` | 2 PASS | Empty-path and expired-deadline calls fail |
| `MezoUniversalRouterFork` | 2 PASS | Invalid-command and expired-deadline calls fail |

The historical name “StatefulFuzz” describes fixed-state randomized quote reads, not a stateful handler campaign. Reverted quote calls are skipped by the conditional output assertion. The Router suite name does not establish funded slippage enforcement: these tests assert only `!ok`, without an exact revert reason or internal trace check. The unusually high UniversalRouter gas use in the earlier transcript further limits conclusions about the rejection cause.

See the [reproduction guide](../protocol-investigations/mezo-reproduction/README.md) for a separate clean-copy run, hashes and exact commands. Mezo results are local fork observations; the existing [Refund Escrow CI](../educational-demos/refund-escrow/CI.md) does not execute them. No confirmed exploitable Mezo finding or economic loss was established. PASS does not prove exhaustive security.

The case-study pages describe the initial disclosure snapshot in which source hashes and harnesses were withheld. The selected public harness is a later evidence addition; its guide and reference record update that availability statement while preserving the original conclusions.

[Portfolio](../README.md)
