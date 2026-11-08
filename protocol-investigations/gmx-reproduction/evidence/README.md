# GMX evidence index

This index separates current verified executions from historical or blocked
records. The files themselves are preserved; the labels below prevent an older
environment failure from being mistaken for the current harness state.

## CURRENT_VERIFIED

| Evidence | File | Result |
|---|---|---|
| EVM educational harness | `evm-sanitized-hardhat.txt` / `.exitcode` | 3 PASS, exit 0 |
| Clean-checkout GMX adaptations | `../GMX_FINAL_TECHNICAL_FIX_RESULTS.md` and the external isolated runner logs | ExternalHandler 2 PASS; OrderVault/DepositVault 2 PASS; total 4 PASS |
| Rust Borrowing model | `rust-model-borrowing-1.89.txt` | `FRESH_MODEL_PASS`, exit 0 |
| Rust Partial Close model | `rust-model-1.89-public-locked.txt` and `.exitcode` | `FRESH_FAIL`, exit 101 on the proposed invariant |

The clean-checkout EVM run uses the official GMX checkout and commit documented
in [`../scripts/reproduce-original-evm.py`](../scripts/reproduce-original-evm.py).
The historical isolated build reported 431 Solidity files; the newer clean
checkout reported 430. These are distinct executions and no cause is inferred
for the difference.

## HISTORICAL_OR_BLOCKED

| Evidence | File | Interpretation |
|---|---|---|
| Earlier Rust public run | `rust-model-public.txt` / `.exitcode` | Historical/blocked lockfile context; not the current Borrowing result |
| Earlier isolated Rust run | `rust-model-1.89-isolated.txt` / `.exitcode` | Historical trace retained for provenance |
| Cargo source snapshot | `Cargo.lock.source-snapshot` | Provenance snapshot, not an execution result |

Anchor execution remains `NOT_REPRODUCED`. The private original EVM suite is
11/11 PASS, while the public package contains only the four disclosure-bounded
adapted tests. The 15-test historical total is not an additional campaign.

## Integrity and provenance

`SOURCE_HASHES.txt`, `LOCKFILE_HASHES.txt` and
[`../THIRD_PARTY_NOTICES.md`](../THIRD_PARTY_NOTICES.md) bind the published
source and dependency claims. The Rust model is third-party source-preserving
code under its retained licence; it is not presented as newly authored GMX
protocol code.
