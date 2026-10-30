# Actual execution record — 30 October 2026

Evidence class: **fresh local execution of a deliberately vulnerable educational artifact**. This is separate from externally validated findings and historical third-party research.

The captured run used Windows AMD64, Forge **1.7.1** (commit `4072e48705af9d93e3c0f6e29e93b5e9a40caed8`), Solc **0.8.24**, Paris EVM, optimizer 200 runs and the seed recorded in [foundry.toml](foundry.toml). There are no third-party Solidity dependencies, forks, wallet keys, RPC calls or production transactions. Python only orchestrates the local commands and records evidence.

| Actual command | Exit code | Observed outcome | Output |
| --- | ---: | --- | --- |
| `forge --version` | 0 | Tool version captured | [Toolchain](evidence/review-2026-10-30/toolchain.txt) |
| `forge fmt --check` | 0 | Formatting check passed | [Formatting](evidence/review-2026-10-30/format.txt) |
| `forge build` | 0 | Compiler run successful | [Build](evidence/review-2026-10-30/build.txt) |
| `forge test -vvv` | 0 | 13 passed, 0 failed, 0 skipped | [Default suite](evidence/review-2026-10-30/default-suite.txt) |
| `forge test --match-contract VulnerableSolvencyCounterexample -vvv`, with profile `counterexample` | **1, expected** | One executed test fails at `INVARIANT_BROKEN: underbacked credits` | [Trace and failure](evidence/review-2026-10-30/intentional-counterexample.txt) |

The default suite includes two fuzz tests of **256 cases each**, and a stateful fixed-version invariant with **128 runs, 8,192 handler calls and zero handler-level reverts**. The call table reports attack 2,697 / deposit 2,742 / refund 2,753 for this recorded seed and toolchain. Counts include potentially empty refund attempts; they are not a claim of 8,192 distinct economic paths. Expected nested-call rejections are caught inside the receiver and are not counted as top-level handler reverts.

The replay script's overall result is PASS only when the normal suite succeeds **and** the vulnerable counterexample fails for the intended reason. Its failure is not hidden, relabeled as a test pass or included in the 13 passing tests.

## Inspect or repeat

- [Machine-readable run and execution-input SHA-256 hashes](evidence/review-2026-10-30/RUN.json)
- [Cross-platform replay runner](scripts/reproduce.py)
- [Recorded source and configuration](README.md)

The runner was invoked against an existing local Forge installation; its machine-specific executable path is omitted from the public record. The displayed Forge arguments and profile values are unchanged. Outputs contain no home-path redactions in this run. The evidence is a self-produced record and can be replaced by an independent rerun; hashes bind files, not authorship or external correctness.

## Source-only replay

A second run copied only `src/`, `test/`, `counterexamples/`, `scripts/` and `foundry.toml` to a new local directory. No project `out/` or `cache/` artifacts, protocol repository or Solidity library were copied. Using the same already-installed toolchain, it produced the same result classifications and identical execution-input hashes: [replay record](evidence/source-only-2026-10-30/RUN.json), [default suite](evidence/source-only-2026-10-30/default-suite.txt), [expected failure](evidence/source-only-2026-10-30/intentional-counterexample.txt).

This checks independence from project build artifacts, not a separate person's review or availability of the compiler on a machine without Foundry. Both runs were performed during this preparation. They are two executions of the same demonstration, not additional findings.

[Limitations and claim map](ANALYSIS.md) · [README](README.md)
