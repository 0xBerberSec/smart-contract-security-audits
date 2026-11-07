# GMX final technical fix results

## Scope

Executed on local branch `gmx-reproducibility-and-evidence-fix`, based on
`origin/main` at `9ee797c05f9f106720b674df8cb8b8a2d6f95f15`. No Audit-OS source,
private original test or other portfolio audit was modified.

## EVM clean-checkout run

The new runner is `scripts/reproduce-original-evm.py`. It cloned
`https://github.com/gmx-io/gmx-synthetics.git`, checked out
`10cfbce6144ade8e9d5cd8c811573955bcd1c132`, installed the official `yarn.lock`
dependencies with `yarn install --frozen-lockfile`, compiled with Hardhat, and
copied only the two public adaptations into `test/guardian/`.

Command:

```text
python protocol-investigations/gmx-reproduction/scripts/reproduce-original-evm.py --workdir <isolated-workdir>
```

Fresh outcome from the script: `FRESH_PASS`.

| File | Result | Exit |
|---|---:|---:|
| `external-handler-safe-controls.spec.ts` | 2 passing | 0 |
| `vault-accounting-observations.spec.ts` | 2 passing | 0 |
| Total | 4 passing | 0 |

The clean Hardhat compilation completed successfully and reported 430 Solidity
files for this checkout/configuration. This is the fresh-run count and is not
silently substituted for the earlier private 431-file build metric.

Run artifacts are retained outside the repository under the isolated workdir.
The script's `summary.json` records the pinned commit and both per-file exit
codes. The script reports PASS only after compilation and both test commands
complete; missing tools, dependency errors or assertion failures return a
non-zero status.

## Rust results

The published model contains the original Borrowing test
`stale_regular_borrowing_update_consumes_closed_duration` at
`solana/model/src/test.rs:1332`. It was executed with:

```text
cargo +1.89 test --locked stale_regular_borrowing_update_consumes_closed_duration -- --nocapture
```

Observed: **1 passed, exit 0**. The log records regular factor `10`, closed
factor `1000`, a six-hour duration and a missed fee factor of `10692000` after
the stale update consumed the clock. This is a model result, not Anchor or
production evidence. Transcript SHA-256:
`D4DBB915962CFD9C9A792BAE919CEF1D91556E3D3F32BAD9C68E685BB9C59880`.

The distinct Partial Close command remains:

```text
cargo +1.89 test --locked partial_close_cannot_extract_more_pnl_than_full_close -- --nocapture
```

Its published result remains **FRESH_FAIL, exit 101**. The assertion was not
changed and the failure is not reclassified as an exploit.

## Public evidence classification

- EVM RefundLedger: educational derivation, 3 PASS.
- GMX ExternalHandler/OrderVault: adapted tests on real GMX components, now clean-checkout reproducible, 4 PASS.
- Original GMX tests: private evidence, 11 PASS; not represented as public executable tests.
- Rust Borrowing: model test, 1 PASS.
- Rust Partial Close: model counterexample, FRESH_FAIL exit 101.
- Anchor: NOT_REPRODUCED.

## Remaining limits

The runner proves a clean local EVM path using the official GMX checkout and
lockfile, but it does not publish the private eleven-test suite. The tests use
Hardhat's local network only; no production RPC, wallet or public transaction
is involved. The Rust PASS and FAIL are model-level evidence and do not prove a
deployed Solana behavior.

## Status

`CLEAN_CHECKOUT_REPRODUCTION_VERIFIED`
