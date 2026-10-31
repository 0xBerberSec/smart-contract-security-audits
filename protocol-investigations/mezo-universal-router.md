# Mezo UniversalRouter — Authorization, Deadlines and Execution Controls

## Objective

Determine whether command execution, callbacks, `allowRevert`, deadlines or residual balances could expose funds to an arbitrary caller.

## Method

The research compared sender/recipient handling, allowance boundaries, invalid paths and expired deadlines. Counter-tests exercised expected reverts and inspected residual-balance hypotheses.

## Evidence and conclusion

The historical fork transcript contains two passing boundary tests. In this phase, the original test body was isolated and reproduced on the same local-fork approach at block 12674080 with Forge 1.7.1. Other phase tests use synthetic actors and injected approvals, so they are fixture evidence only. No production-funded residual balance or user-funded drain was demonstrated. This is a false-positive elimination study, not a confirmed vulnerability.

## Reproduction record and limits

| Suite | Recorded result | Assertion actually used |
| --- | --- | --- |
| `MezoRouterSlippageAndDeadline` | 2 PASS | Low-level calls with an empty path or expired deadline return failure |
| `MezoUniversalRouterFork` | 2 PASS | Low-level calls with an invalid command or expired deadline return failure |

The environment used Forge 1.7.1 (`4072e48705af9d93e3c0f6e29e93b5e9a40caed8`), Solidity 0.8.24, London, and a local Mezo fork at block 12674080 (chain ID 31612). The original test bodies were copied unchanged; deployed code was loaded from the fork. No public transaction was sent.

These assertions check only `!ok`, not a specific revert selector or trace location. They therefore do not by themselves prove why execution failed, that a particular authorization check ran, or that rejection occurred before a transfer. The UniversalRouter transcript also reports unusually high gas use; its PASS labels must not be treated as proof of the intended internal rejection branch.

The Router suite name mentions slippage, but these two reproduced tests do not exercise a funded swap or minimum-output enforcement. Callbacks, reentrancy, allowance theft and residual recovery were research questions; the reproduced tests do not establish their safety. No actual user-funded residual loss was demonstrated.

**No confirmed exploitable vulnerability was established by these experiments.** Raw execution evidence remains private; no public executable Mezo reproduction or hosted Mezo CI is claimed.
