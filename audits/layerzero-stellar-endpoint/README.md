# LayerZero Stellar Endpoint - ULN302 DVN Runtime Budget Mismatch

[Back to portfolio](../../README.md)

## Finding

**Title:** ULN302 accepts DVN configurations that are valid under validation but non-executable at runtime, causing deterministic `quote()` / `send()` failure under Soroban budget limits.

**Contest:** [LayerZero - Stellar Endpoint](https://code4rena.com/audits/2026-04-layerzero-stellar-endpoint/dashboard)  
**Submission:** [S-1284](https://code4rena.com/audits/2026-04-layerzero-stellar-endpoint/submissions/S-1284)  
**Severity:** Low  
**Status:** Validated  
**Researcher:** 0xBerberSec

## Summary

ULN302 validates `required_dvns` and `optional_dvns` independently, but runtime paths process both lists as a combined set.

Validation checks each list separately:

```text
required_dvns.len() <= MAX_DVNS
optional_dvns.len() <= MAX_DVNS
```

Runtime behavior iterates over:

```text
required_dvns.iter().chain(optional_dvns.iter())
```

On Soroban, transaction budget limits are strict. This creates a mismatch where a configuration can be accepted as valid but later become non-executable during `quote()` or `send()`, causing deterministic budget exhaustion.

## Root Cause

The configuration validation logic does not enforce a combined bound across required and optional DVNs.

The runtime paths consume both sets together:

- `quote_dvns()`
- `assign_dvns()`

Each DVN can trigger external work such as fee calculation or job assignment. A sufficiently large combined DVN set can exceed Soroban runtime limits even though each individual list passed validation.

## Impact

This can cause route-level messaging unavailability:

- `quote()` fails
- `send()` fails
- OApps cannot send messages through the affected route
- Recovery requires privileged reconfiguration

This is not only a gas-cost concern. The core issue is that protocol validation can accept configurations that runtime execution cannot complete.

## Mitigation

Validation should reflect runtime behavior by enforcing a combined bound:

```text
total_dvns = required_dvns.len() + optional_dvns.len()
assert(total_dvns <= MAX_COMBINED_DVNS)
```

The combined limit should be chosen so that accepted configurations remain executable under Soroban budget constraints.

## Proof of Concept

See [PoC.md](./PoC.md).

## Links

- [Code4rena submission S-1284](https://code4rena.com/audits/2026-04-layerzero-stellar-endpoint/submissions/S-1284)
- [LayerZero Stellar Endpoint audit dashboard](https://code4rena.com/audits/2026-04-layerzero-stellar-endpoint/dashboard)

[Back to portfolio](../../README.md)
