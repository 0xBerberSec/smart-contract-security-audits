# Mezo — minimal historical-fork reproduction

This harness exposes the bounded Pool quote/accounting and Router rejection checks discussed in the [Pool](../mezo-pool-fuzz.md) and [UniversalRouter](../mezo-universal-router.md) studies. **Confirmed Mezo exploitable findings: 0.** It is independent research, not a client audit or security certification.

This is a follow-up evidence package. The earlier case-study pages describe the initial disclosure snapshot, when the harness and input hashes were withheld. This package updates their public-availability note by supplying the five selected harness files and a sanitized reference record; full dossiers and raw transcripts remain private.

## Recorded environment

- Forge and Anvil **1.7.1**, commit `4072e48705af9d93e3c0f6e29e93b5e9a40caed8`.
- Solidity **0.8.24**, EVM **London**.
- Mezo block **12674080**, chain ID **31612**.
- Expected recorded block hash: `0x202a0c344bdf6230ba40ec63b09a9dc51de8342a58844fc1f202b7f36ed5f913`.
- Python 3.10+ and an archive-capable Mezo RPC are required. No wallet, private key, signature or third-party Solidity library is needed.

## Run from a clean checkout

Install the recorded tools separately and place `forge` and `anvil` on PATH. Set your RPC URL only in the process environment. The placeholder below is not a usable endpoint; choose a provider authorized for your use.

PowerShell:

```powershell
cd protocol-investigations/mezo-reproduction
$env:MEZO_RPC_URL="<YOUR_MEZO_ARCHIVE_RPC>"
python -B scripts/reproduce.py
```

POSIX:

```bash
cd protocol-investigations/mezo-reproduction
export MEZO_RPC_URL="<YOUR_MEZO_ARCHIVE_RPC>"
python3 -B scripts/reproduce.py
```

`FORGE_BIN` and `ANVIL_BIN` may name already-installed executables if PATH does not contain them. Do not put credentials in tracked files. Tool/compiler downloads may require public network access.

The runner checks both tool versions, reads the upstream chain ID and historical header, starts its own Anvil on `127.0.0.1:8546`, verifies the local header and target code, then invokes Forge against that local fork. Port 8546 must be free; existing services are neither reused nor stopped. The original Pool setup intentionally skips a nested fork for this exact local URL, so the outer `--fork-url` is required. Router setup creates a nested fork of the same pinned local node. The runner sends upstream reads only and does not sign or broadcast public transactions.

The temporary `RUN.json` and transcripts are written outside the checkout; the printed directory is unique for each invocation. The record contains commands, input SHA-256 values, block hash, suite status and tool versions. RPC URLs and home-directory prefixes are redacted from captured text. Inspect outputs before sharing them. An RPC/setup error is BLOCKED rather than a passing experiment; a failed suite is FAILED. The runner closes only its own Anvil process.

## Actual clean-copy execution — 31 October 2026

| Suite | Observed result | Assertion boundary |
| --- | --- | --- |
| `MezoPoolBoundedStatefulFuzz` | 1 PASS; 1,024 runs | Successful quote output <= selected reserve; conditional zero-input check |
| `MezoPoolAccountingInvariants` | 2 PASS | Balance/reserve coverage, LP floor, reserve-view agreement, deterministic zero-input checks |
| `MezoRouterSlippageAndDeadline` | 2 PASS | Low-level calls with expired deadline or empty path return failure |
| `MezoUniversalRouterFork` | 2 PASS | Low-level calls with expired deadline or invalid command return failure |

The [reference record](evidence/reference-run.json) is a self-recorded Windows run from a clean source-only copy. All seven test functions executed; the fuzz function used 1,024 cases. Raw transcripts remain privately retained. This is separate from the earlier private reproduction and historical logs. There is no hosted Mezo CI claim; [Refund Escrow CI](../../educational-demos/refund-escrow/CI.md) executes the educational lab only.

## Source fidelity and technical adaptations

The four test files and `MezoPoolForkBase.sol` match the selected private reproduction copies after normalizing CRLF line endings to LF. No assertion, target address, setup body or protocol logic was changed. They are MIT-marked harness files; deployed protocol implementations are loaded through the fork and are not redistributed here.

Project scaffolding changed: `tests/` became `test/`; a minimal `foundry.toml` selects those files; the runner pins the historical fork and a recorded fuzz seed. The original relative imports remain unchanged. The only Solidity file adaptation is line-ending normalization; `.gitattributes` pins LF so future Windows and POSIX checkouts use the same bytes. The reference record supplies exact hashes for these normalized public inputs; the private fidelity record retains both original and public hashes. There is no claimed deployment source commit match beyond the forked code and historical block.

This clean-copy run uses seed `0x0000000000000000000000000000000000000000000000000000000000000005`. Earlier private run seeds were not uniformly recorded; matching PASS counts does not imply the same 1,024 generated inputs. The runner checks the expected historical block hash and selected effective compiler/configuration fields before starting the suites.

The Router test contains public token identifiers and public on-chain addresses as route inputs. It contains no private lending analysis or private signature. Calls use zero input amounts for rejection checks; they do not finance an actor, obtain a role, approve a spender or test a profitable swap. Anvil's local account state does not establish real-world attacker funding.

## What PASS does and does not show

- The fuzzer's historical name includes “Stateful”, but it performs randomized **fixed-state read-only quotes**, not a stateful handler campaign. Reverted quotes skip the conditional output assertions; the harness does not count how many quotes succeeded. Amounts are bounded by the input reserve. Passing is not exhaustive state or amount coverage.
- Accounting checks cover three hard-coded Pools. A failed balance query for the first asset falls back to native balance without independently proving that the asset is the native precompile. The LP floor check tests the number, not ownership or permanent locking.
- Router tests assert only `!ok`. They do not match the revert selector or prove that authorization/deadline validation caused failure. Even an unrelated failure could satisfy those assertions. The observed UniversalRouter gas values are unusually high and remain an interpretation limit.
- Despite the suite/test names, these checks do not establish funded slippage enforcement, pre-transfer rejection ordering, callback safety, allowance safety or residual-balance security. No user loss, profit or permissionless extraction was demonstrated.

The source is published to let reviewers inspect these limitations directly. A passing harness is not a finding, an endorsement of the protocol or proof of all production behavior.

## Correction and complementary tests — 1 November 2026

**The historical UniversalRouter tests under London returned PASS because the calls halted with `NotActivated`. They did not establish the intended deadline/invalid-command mechanisms.** Their files and results above remain unchanged. The high gas values are not production swap-cost estimates.

Use the separate [complementary reproduction guide](supplemental/README.md) for measured quote coverage and exact Router errors. Its configuration and runner are deliberately separate from this historical project: eight substantive controls plus one logger PASS locally under Cancun; the same controls expose two failures under London. Four historical upstream read-only calls corroborate the selected UniversalRouter outcomes. This does not establish complete historical hardfork or native-precompile equivalence.

The historical accounting comment about an unavailable BTC ERC20 selector describes a failed local read, not the network: upstream `balanceOf` succeeds at this block. The new tests verify asset identity explicitly and qualify the minimum LP holder check. No old assertion or reference result was changed.
