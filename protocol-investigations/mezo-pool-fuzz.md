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

The [selected test sources, Foundry configuration and reproduction runner](mezo-reproduction/README.md) are available for local replay, subject to a compatible historical Mezo RPC and the recorded toolchain. The [sanitized reference record](mezo-reproduction/evidence/reference-run.json) provides selected environment, result and input-hash information. Raw historical transcripts and the complete research dossier remain private; the reference record is not an independently verified execution transcript.

The published harness retains the fixed-state quote checks described above: 1,024 randomized quote cases are not a complete stateful campaign. No hosted Mezo CI is claimed. The separate [Refund Escrow CI](../educational-demos/refund-escrow/CI.md) covers the educational lab only.

## Complementary evidence — 1 November 2026

The historical conditional fuzzer is preserved. A separate [coverage harness](mezo-reproduction/supplemental/README.md) addresses its non-vacuity limitation on the same three Pools at block 12674080:

- Ten deterministic amounts per Pool/direction: **60/60 valid quote returns**, 48 strictly positive outputs, zero rejected calls and zero invalid return payloads.
- **1,024 material-input fuzz cases**, each visiting all six Pool/direction combinations: **6,144 asserted successful quote calls**, not 6,144 independent fuzz scenarios or necessarily unique inputs.
- Quote success, positive output and reserve bounds are mandatory in that material-input domain; reverts cannot silently skip assertions. Zero/dust inputs are separately recorded in the grid.
- A 20-point monotonicity sample in both directions covers only the volatile Pool at fixed reserves and fee.

These remain fixed-state read experiments, not swaps or a stateful handler campaign. The material-input interval is explicitly `[ceil(inputReserve/1e6), inputReserve]`; it is not arbitrary-input coverage.

Accounting complements identify the canonical BTC address before comparing native balance and locate the minimum LP balance at address(1), with ordinary unauthorized-transfer rejection checks. A failed local ERC20 read does not identify an asset as native: the BTC facade's balance query succeeds on upstream Mezo at this block. A live 1,000-unit holder balance is not an absolute proof of permanent locking. See the guide for precompile and authorization limits.
