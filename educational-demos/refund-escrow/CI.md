# Reproduction checks and CI status

The [workflow](../../.github/workflows/refund-escrow.yml) is prepared for Ubuntu 24.04 and Windows Server 2022 with Python 3.12 and Forge **1.7.1** (`4072e48705af9d93e3c0f6e29e93b5e9a40caed8`). Solidity 0.8.24, Paris, optimizer 200 and the experiment budgets remain pinned in [foundry.toml](foundry.toml).

**GitHub CI status: not yet executed for this corrective workflow.** Local runs are not relabeled as successful GitHub Actions runs. After publication, inspect the actual PR checks before claiming a green CI result.

Local corrective verification on 30 October 2026 passed on native Windows (Python 3.13.5) and Ubuntu under WSL (Python 3.12.3). Both ran the 13 Python orchestration regressions and the real Foundry replay, with identical current execution-input hashes and effective configuration. Windows used a fresh public clone with the uncommitted corrective files applied to a disposable index and rematerialized with `core.autocrlf=true`; Linux used a source-only copy without build/cache artifacts. These are local checks of the candidate, not hosted-runner or third-party attestations.

Each matrix job runs:

```sh
python -B -m unittest discover -s scripts/tests -v
python -B scripts/reproduce.py --output "<runner temporary directory>/refund-escrow-evidence"
```

The first command checks Python orchestration with mocked Forge responses. The second executes real Forge version/configuration checks, formatting, build, the complete passing suite (including two 256-case fuzz tests and the 128-run invariant), and the separate expected safety-property failure. An arbitrary nonzero exit is not accepted as the counterexample.

The runner records only a whitelist of effective configuration fields; full Forge configuration can contain paths or credentials. Outputs are retained as a short-lived artifact for inspection. No wallet, RPC, deployment, production transaction or project secret is required. Initial tool/compiler downloads require public network access.

Workflow permissions are `contents: read`; checkout does not persist credentials. Action implementations are pinned to full commit SHAs, Foundry to a named release, and Foundry caching is disabled. The workflow uses normal `pull_request`, not privileged `pull_request_target`. Runner images and Python 3.12 patch versions can evolve; this is not a fully hermetic build.

The original [execution records](EVIDENCE.md) remain historical evidence for their exact input hashes. No Solidity behavior or test budget was changed by the orchestration fixes. The LF attributes address the Windows formatting failure; existing output directories are preserved by creating new unique children.
