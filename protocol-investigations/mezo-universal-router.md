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

**No confirmed exploitable vulnerability was established by these experiments.** The [selected test sources and runner](mezo-reproduction/README.md) support local replay with a compatible historical Mezo RPC. The [sanitized reference record](mezo-reproduction/evidence/reference-run.json) reports selected results and input hashes. Raw private transcripts are not presented as published evidence, and no hosted Mezo CI is claimed.

Source availability does not strengthen the original assertions: the tests primarily check `!ok`, not the exact internal revert cause. They do not demonstrate a complete funded swap, slippage enforcement or economic exploit. Reviewers can inspect these limits directly in the selected harness.

## Technical correction — 1 November 2026

The old UniversalRouter tests under London genuinely returned PASS, but both calls halted with **`NotActivated`**. Those PASS labels are **not evidence of the intended deadline or command controls**. The deployed runtime contains newer instructions (PUSH0 and transient-storage operations) unsupported by London. The unusually high reported gas is an exceptional-halt/harness symptom, not a production execution cost. Historical test files, runner, reference record and results are retained unchanged.

A separate [complementary harness](mezo-reproduction/supplemental/README.md) checks exact error data and a successful empty execution under an explicitly selected Cancun profile. Nine functions PASS there: eight asserted controls and one diagnostic logger. Under London, the same suite has seven PASS and two FAIL; these failures expose the environment mismatch rather than a protocol vulnerability.

Four separate upstream historical RPC `eth_call` observations at block 12674080 corroborate the empty execution and the exact errors `TransactionDeadlinePassed()`, `LengthMismatch()` and `InvalidCommandType(63)`. These are read-only simulations on historical state, not public transactions. Cancun is a compatible-opcode local environment; full historical Mezo hardfork/native-precompile equivalence has not been established. The nine local PASS labels are not nine production validations.

Ordinary Router controls also check exact `Expired()`, `InvalidPath()` and `InsufficientOutputAmount()` errors. The minimum-output case first obtains a positive live-route quote, then sets the minimum to quote+1 and checks unchanged token balances/allowance. This exercises the selected output-minimum rejection, without claiming a funded successful swap or deriving validation order from rollback alone.

Callbacks, authorization of funded transfers, allowance safety and residual-balance security remain outside these reproduced assertions. **No confirmed exploitable vulnerability was established by these experiments.**

### Historical hardfork evidence and final isolation check — 1 November 2026

The official [Mezo v7.0.0 release](https://github.com/mezo-org/mezod/releases/tag/v7.0.0) places the Cancun compatibility upgrade at block 7,691,500, before this fixture's block 12,674,080. This supports Cancun as a historically relevant compatibility level; it does not establish complete equivalence with standard Foundry or identify the precise client binary used at the fixture block.

A separate raw-call experiment removed compiler differences: identical deployed runtime bytes, block/hash, calldata, explicit caller, zero value and 500,000 gas were used on London and Cancun nodes. London stopped at runtime offset **176: PUSH0**, reporting `NotActivated`; Cancun completed the empty execution and returned the expected errors for the negative cases. Four matching upstream simulations with the same explicit caller were repeated. The same provider supplied the historical data; this is execution corroboration, not independent-provider validation.

The Forge comparison alone changes both compiler code generation and execution rules, so the compiled harness binaries differ. The raw-call comparison establishes that the deployed target failure does not depend on recompiling the harness. The supplemental runner now explicitly pins its Anvil state backend to Cancun for both profiles; Forge's selected EVM profile governs the test execution. No test assertion was changed.
