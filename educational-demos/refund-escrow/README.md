# Refund Escrow — an executable audit-method demonstration

**Educational artifact, created with Codex assistance on 30 October 2026.** This is an original teaching protocol containing a deliberately introduced instance of the well-known refund reentrancy pattern. It is independent of private research. It is not a vulnerability discovered in a third-party protocol, a contest finding or production-ready code. Do not deploy it with real funds.

The runnable experiment connects each stage of my method to code and evidence:

**Value Flow → Security Invariants → Targeted Tests → Counter-Tests → Economic Impact Qualification**

## Start in three minutes

1. Compare the two `refund()` implementations in [RefundEscrow.sol](src/RefundEscrow.sol).
2. Read the [hypothesis and economic accounting](ANALYSIS.md).
3. Inspect the [callback test and counter-tests](test/RefundEscrow.t.sol).
4. Read the [actual execution record](EVIDENCE.md), including the intentionally failing safety assertion and the fixed-version invariant campaign.

## Reproduce locally

Use Foundry; the recorded run used **Forge 1.7.1**, **Solc 0.8.24** and the **Paris EVM**. Solidity compiler, seed, fuzz runs and invariant budget are pinned in [foundry.toml](foundry.toml). No forge-std, submodule, RPC URL, wallet, API key or live chain is needed. The first compilation may download the pinned compiler if not cached.

The recorded Forge version is available as the [official v1.7.1 release](https://github.com/foundry-rs/foundry/releases/tag/v1.7.1). The tool itself is not vendored in this repository.

From `educational-demos/refund-escrow`:

```sh
forge fmt --check
forge build
forge test -vvv
```

The default suite must report **13 passing tests**: 10 deterministic tests, two fuzz tests with 256 cases each, and one fixed-contract stateful invariant with 128 runs and 8,192 handler calls. This is a bounded local experiment, not a comprehensive security proof.

To run the separate vulnerable safety property on POSIX shells:

```sh
FOUNDRY_PROFILE=counterexample forge test --match-contract VulnerableSolvencyCounterexample -vvv
```

That command must exit **1** with `INVARIANT_BROKEN: underbacked credits`. It proves the vulnerable version fails the safety property; it is deliberately outside the default passing suite. On PowerShell, set `$env:FOUNDRY_PROFILE='counterexample'` for that command and remove the variable afterwards.

For a cross-platform replay that checks both the passing suite and the specific expected failure, using Python 3.10+ from the standard library:

```sh
python scripts/reproduce.py
```

The [runner](scripts/reproduce.py) requires the recorded Forge 1.7.1 release and checks the effective compiler, profiles, seed, fuzz/invariant budgets and both 256-case fuzz results. It removes inherited Foundry/Dapp overrides, checks each exit code and the exact counterexample failure reason. `--forge` can select an existing Forge binary. Timing and gas may vary by platform.

`--output` accepts a path relative to this lab, or an absolute path outside the repository. Quote paths containing spaces. A missing directory is created; an existing directory receives a new unique `run-*` child, preserving all previous files. An existing file is rejected before Forge runs. With no option, a unique directory is created under `evidence/`.

```sh
python scripts/reproduce.py --output "evidence/local review"
python -B -m unittest discover -s scripts/tests -v
```

The Python regression tests mock Forge output to check orchestration, paths and rejection conditions. They are separate from the 13 real Foundry tests and do not prove Solidity behavior.

The repository's [attributes](../../.gitattributes) keep the lab's Solidity, Python and TOML files in LF even with Windows `core.autocrlf=true`. This preserves formatting and byte-level hashes. For an older checkout already converted to CRLF, use a fresh clone after the attributes change is published; do not overwrite local research to refresh files.

[CI configuration and execution status](CI.md) describes the proposed Linux/Windows checks. A prepared workflow is not evidence of a successful GitHub run.

## Evidence boundaries

The captured [local record](evidence/review-2026-10-30/RUN.json) identifies executed inputs by SHA-256. The [raw default-suite output](evidence/review-2026-10-30/default-suite.txt) and [counterexample trace](evidence/review-2026-10-30/intentional-counterexample.txt) are available. These are self-produced local execution artifacts, not independent attestation.

A [source-only replay](evidence/source-only-2026-10-30/RUN.json), with no copied project build/cache artifacts, reproduced the result classifications with matching input hashes. See [the execution record](EVIDENCE.md#source-only-replay) for scope.

The actor set and handler actions are deliberately narrow. Handler call counts include possible no-op refund attempts, and zero reported handler reverts does not mean nested calls never reverted. The fixed receiver test deliberately catches the nested refund rejection. No fees, prices, token quirks, upgrades, cross-function callback deposits, MEV, gas-profit model or live deployment is tested.

[Research process and AI contribution](RESEARCH_PROCESS.md) · [Analysis and limitations](ANALYSIS.md) · [Portfolio](../../README.md) · [MIT license](../../LICENSE)
