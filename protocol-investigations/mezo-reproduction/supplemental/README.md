# Mezo complementary evidence and technical correction

**1 November 2026. Independent research, not a client audit, certification or validated vulnerability. Confirmed exploitable Mezo findings: 0.**

## Historical correction

The [historical UniversalRouter tests](../test/MezoUniversalRouterFork.t.sol) under **London** returned PASS because both low-level calls halted with **`NotActivated`**. Those PASS labels did not verify the intended deadline or invalid-command checks. The five historical Solidity files, configuration, runner and reference record are preserved unchanged.

The deployed UniversalRouter runtime contains PUSH0 and transient-storage instructions unsupported by London. Its SHA-256 at the selected block is `949ac3536c8fe99912ade4f65ccea1161230186f0dba83bede70d263083554d3`. Historical gas near 1.057 billion is an exceptional-halt/harness symptom, not a production transaction estimate.

This separate project uses **Cancun** to support the observed opcodes. Full historical Mezo hardfork/native-precompile equivalence is **not established**. Four separate upstream read-only historical RPC simulations corroborate only these selected UniversalRouter outcomes:

| Input | Observed outcome |
| --- | --- |
| Empty commands/inputs, future deadline | Success, empty return |
| Empty commands/inputs, deadline zero | `TransactionDeadlinePassed()` / `0x5bf6f916` |
| One command, zero inputs | `LengthMismatch()` / `0xff633a38` |
| Command `0x3f`, one empty input | `InvalidCommandType(63)` / `0xd76a1e9e` + encoded 63 |

The [selected evidence record](evidence/verified-upgrade.json) includes the complete four calldata/result pairs. These are public-data simulations, not transactions. Their RPC default caller differs from the local harness caller. Agreement on these calls does not prove all caller-sensitive behavior. The same public provider supplied the fork data; these are separate executions, not independent-provider validation.

## What executes

[MezoEvidenceControls.t.sol](test/MezoEvidenceControls.t.sol) is byte-identical to the reviewed complementary test source. It is a self-contained harness; no protocol implementations or third-party libraries are redistributed.

| Function | Assertion or observation |
| --- | --- |
| `testQuoteGridCoverage` | Ten valid quotes on each of six Pool/direction paths; counts successes, rejections, invalid returns and positive outputs |
| `testFuzzMaterialQuotesAllPairs` | Every case executes six mandatory quotes; each output is positive and bounded by the selected reserve |
| `testVolatileQuoteMonotonicity` | Twenty ordered amounts in each direction on the volatile Pool; nondecreasing outputs |
| `testNativeIdentityAndMinimumLpHolder` | Explicit canonical BTC identity, native balance/reserve equality, LP holder at address(1), ordinary unauthorized-transfer rejection |
| `testRouterExactDeadlineAndPath` | Exact `Expired()` and `InvalidPath()` payloads |
| `testRouterExactSlippageWithQuoteControl` | Positive route quote, minimum=quote+1, exact `InsufficientOutputAmount()`, unchanged balances/allowance |
| `testUniversalRouterPositiveEmptyExecution` | Successful empty execution with a future deadline |
| `testUniversalRouterExactErrors` | Exact three errors in the table above |
| `testUniversalRouterDiagnostic` | Logs a gas-bounded empty-execution result; **no security assertion** |

Fresh source-only-copy result: **9 PASS, 0 failed, 0 skipped under Cancun: eight asserted functions plus one logger**. Under London the same suite returns **7 PASS, 2 FAIL**. The positive execution and exact-error failures under London are retained diagnostics; they are not a protocol finding.

### Quote scope and coverage

Three fixed Pools, two directions each, ten amounts per path produced **60 valid successes, zero rejections/invalid returns and 48 positive outputs**. The grid includes zero, 1, 2, floor(R/1e6), floor(R/1e4), floor(R/100), floor(R/10), floor(R/2), R and R+1, where R is the input reserve. Twelve zero outputs are retained as observations rather than removed. The minimum coverage is a regression expectation from preflight of this exact historical fixture, not a universal obligation for all amounts or reserves.

Fuzzing runs **1,024 cases** with seed 1. Each case visits all six paths, giving **6,144 asserted quote calls**, not 6,144 independent fuzz scenarios or necessarily distinct amounts. Domain per path: `[ceil(R/1e6), R]`; zero/dust is handled by the separate grid. Reverts fail the material-input test instead of bypassing assertions. These are fixed-state reads, not swaps or a stateful handler.

The monotonicity sample concerns only the volatile Pool with its fixed fee/reserves. The facade at the canonical BTC address can answer `balanceOf` on upstream Mezo; local failure is not evidence of an absent ERC20 selector. The LP holder check is bounded authorization evidence, not proof of permanent unspendability.

## Reproduce separately from the historical harness

Requirements: Forge and Anvil **1.7.1**, commit `4072e48705af9d93e3c0f6e29e93b5e9a40caed8`; Solidity **0.8.24**; Python 3.10+; authorized historical Mezo RPC. This complementary harness enables optimizer/200 runs; its configuration does not overwrite the original London project.

The runner verifies chain **31612**, block **12674080**, hash `0x202a0c344bdf6230ba40ec63b09a9dc51de8342a58844fc1f202b7f36ed5f913`, tool versions, effective configuration and UniversalRouter runtime hash. It starts its own local Anvil with zero generated accounts on port 8562, checks the local block, and stops only that process. No wallet, signature or public transaction is used.

From the parent `mezo-reproduction` directory:

```powershell
$env:MEZO_RPC_URL="<YOUR_AUTHORIZED_ARCHIVE_RPC>"
python -B scripts/reproduce_supplemental.py
# Diagnostic: expected nonzero FAILED, not PASS
python -B scripts/reproduce_supplemental.py --evm london
```

```bash
export MEZO_RPC_URL="<YOUR_AUTHORIZED_ARCHIVE_RPC>"
python3 -B scripts/reproduce_supplemental.py
# Diagnostic: expected nonzero FAILED, not PASS
python3 -B scripts/reproduce_supplemental.py --evm london
```

`FORGE_BIN` and `ANVIL_BIN` may point to existing executables. A missing RPC produces **BLOCKED / exit 2**, not a green run. A failed assertion or incomplete test record produces **FAILED / exit 1**. Successful validation requires the nine exact function names, no missing/duplicate/skipped tests and the 1,024-case fuzz count. Temporary `RUN.json` and sanitized transcripts are written outside the checkout. Review outputs before sharing; no redaction scheme guarantees that arbitrary future logs contain no sensitive material.

Offline parser/privacy controls are separate from protocol execution:

```bash
python3 -B -m unittest discover -s scripts -p test_supplemental_runner.py -v
```

Eight unit tests PASS, using synthetic parser strings. These are not eight additional Mezo experiments. No hosted Mezo fork CI is claimed; the existing Refund Escrow workflow remains separate.

## Economic and coverage limits

There is no funded successful swap positive control, no general callback/allowance/residual-balance proof, no full chain-native-precompile reproduction, and no exhaustive reserve-state or amount coverage. Rejected-state equality alone does not prove validation ordering. Private economic fixtures are not distributed. No security finding, user loss or permissionless profit is claimed.

### Historical hardfork evidence and final isolation check — 1 November 2026

The official [Mezo v7.0.0 release](https://github.com/mezo-org/mezod/releases/tag/v7.0.0) places the Cancun compatibility upgrade at block 7,691,500, before this fixture's block 12,674,080. This supports Cancun as a historically relevant compatibility level; it does not establish complete equivalence with standard Foundry or identify the precise client binary used at the fixture block.

A separate raw-call experiment removed compiler differences: identical deployed runtime bytes, block/hash, calldata, explicit caller, zero value and 500,000 gas were used on London and Cancun nodes. London stopped at runtime offset **176: PUSH0**, reporting `NotActivated`; Cancun completed the empty execution and returned the expected errors for the negative cases. Four matching upstream simulations with the same explicit caller were repeated. The same provider supplied the historical data; this is execution corroboration, not independent-provider validation.

The Forge comparison alone changes both compiler code generation and execution rules, so the compiled harness binaries differ. The raw-call comparison establishes that the deployed target failure does not depend on recompiling the harness. The supplemental runner now explicitly pins its Anvil state backend to Cancun for both profiles; Forge's selected EVM profile governs the test execution. No test assertion was changed.
