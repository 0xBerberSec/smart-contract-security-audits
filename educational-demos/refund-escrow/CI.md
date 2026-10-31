# Reproduction checks and CI status

The [workflow](../../.github/workflows/refund-escrow.yml) targets Ubuntu 24.04 and Windows Server 2022 with Python 3.12 and Forge **1.7.1** (`4072e48705af9d93e3c0f6e29e93b5e9a40caed8`). Solidity 0.8.24, Paris, optimizer 200 and the experiment budgets remain pinned in [foundry.toml](foundry.toml).

## Verified hosted snapshot — 31 October 2026

[GitHub Actions run 41161196989](https://github.com/0xBerberSec/smart-contract-security-audits/actions/runs/41161196989), triggered by the push to `main` at commit [`5da055c28753180bfaa486e36ce1e66186777c0e`](https://github.com/0xBerberSec/smart-contract-security-audits/commit/5da055c28753180bfaa486e36ce1e66186777c0e), completed successfully on **both Linux and Windows** after [PR #3](https://github.com/0xBerberSec/smart-contract-security-audits/pull/3). The job steps, execution records and input hashes were inspected; local checks are not substituted for hosted results.

| Check in that hosted snapshot | Ubuntu 24.04 | Windows Server 2022 |
| --- | --- | --- |
| Python version | 3.12.15 | 3.12.10 |
| Python orchestration regressions | 13 passed | 13 passed |
| Forge formatting and build | Exit 0 | Exit 0 |
| Foundry suite | 13 passed, 0 failed, 0 skipped | Same |
| Fuzz tests | Two tests, 256 cases each | Same |
| Fixed-version invariant | 128 runs, 8,192 handler calls, 0 handler-level reverts | Same |
| Vulnerable safety-property counterexample | Exit 1 at the expected `INVARIANT_BROKEN` assertion | Same |

These results apply to the linked revision. Later runner or test changes require a new CI run; the existing green run does not attest to them. Two additional regressions now cover timeout-error redaction and missing configuration, so the current Python suite has **15 tests**. They are not part of the 13-test hosted snapshot above.

Each matrix job runs:

```sh
python -B -m unittest discover -s scripts/tests -v
python -B scripts/reproduce.py --output "<runner temporary directory>/refund-escrow-evidence"
```

The first command checks Python orchestration with mocked Forge responses. The second executes real Forge version/configuration checks, formatting, build, the complete passing suite (including two 256-case fuzz tests and the 128-run invariant), and the separate expected safety-property failure. An arbitrary nonzero exit is not accepted as the counterexample.

The 13 passing Foundry tests comprise ten deterministic tests, two fuzz tests and one invariant. The 8,192 calls belong to one bounded campaign of 128 runs, include possible no-op actions and are not an exhaustive security proof. A caught nested rejection is not counted as a handler-level revert. The separate counterexample is an expected failure, not an extra passing test.

The runner records only a whitelist of effective configuration fields; full Forge configuration can contain paths or credentials. Outputs are retained as a short-lived artifact for inspection. No wallet, RPC, deployment, production transaction or project secret is required. Initial tool/compiler downloads require public network access.

Workflow permissions are `contents: read`; checkout does not persist credentials. Action implementations are pinned to full commit SHAs, Foundry to a named release, and Foundry caching is disabled. The workflow uses normal `pull_request`, not privileged `pull_request_target`. Runner images and Python 3.12 patch versions can evolve; this is not a fully hermetic build.

The original [execution records](EVIDENCE.md) remain historical evidence for their exact input hashes and runner revision. Local preparation results from 30 October concern the earlier candidate, not the current runner/tests. No Solidity behavior or Foundry test budget was changed by these error-handling fixes. The LF attributes address the Windows formatting failure; existing output directories are preserved by creating new unique children.
