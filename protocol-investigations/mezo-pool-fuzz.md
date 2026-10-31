# Mezo Pool — Reserve Invariants and Bounded Fuzzing

## Objective

Check whether quote logic could return more output than the selected reserve or behave inconsistently at zero and boundary inputs.

## Invariants

- Successful output must not exceed the selected output reserve.
- Zero input must produce zero output.
- Reserve and balance transitions must remain economically coherent.

## Method and evidence

The existing harness was historically executed at 16, 32, 128, 256 and 1,024 fuzz runs. The 1,024-run transcript contains an explicit `Ran 1 test` and `PASS`. In this phase, the original test body and base contract were isolated without protocol changes and reproduced on a local fork at Mezo block 12674080 with Forge 1.7.1. Separate historical tests covered empty pools, decimals, hostile-token behavior and reserve/balance boundaries.

## Result

No counterexample, user loss or permissionless extraction was demonstrated. The result is a bounded local property, not proof of all production swap sequences. The current reproduction is evidence of the selected harness and historical fork state only.

## Reproduction record

| Item | Recorded result |
| --- | --- |
| Forge | 1.7.1, commit `4072e48705af9d93e3c0f6e29e93b5e9a40caed8` |
| Solidity / EVM | 0.8.24 / London |
| Mezo fork | Block 12674080, chain ID 31612 |
| `MezoPoolBoundedStatefulFuzz` | PASS, 1,024 runs |
| `MezoPoolAccountingInvariants` | 2 tests PASS |
| Public transactions | None sent during these experiments |

The harness and its base were copied unchanged into an isolated Foundry project. The Pool implementations were read from the historical fork rather than rebuilt from source. Accounting checks covered three fixed Pool addresses, reserve-view consistency, balance coverage and the minimum LP supply check. A failed token balance query for the first asset falls back to native balance, so this is not general validation of arbitrary token behavior.

Despite its name, `MezoPoolBoundedStatefulFuzz` performs randomized read-only quotes against fixed state; it is not a stateful handler campaign. Inputs select a Pool, swap direction and amount bounded by the input reserve. The output assertion is conditional on a successful quote; reverted quotes are not failures in this harness. A separate deterministic test checks zero-input quotes.

The transcripts and input hashes are retained privately. The public repository does not ship the Mezo reproduction harness or hosted Mezo CI. The separate [Refund Escrow CI](../educational-demos/refund-escrow/CI.md) covers the educational lab only.
