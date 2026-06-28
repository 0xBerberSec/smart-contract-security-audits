# Jupiter Lend - QA Research

[Back to portfolio](../../README.md)

## Report

**Title:** Closed Branch Can Be Reassigned as Active Without Lifecycle Validation  
**Contest:** [Jupiter Lend](https://code4rena.com/audits/2026-02-jupiter-lend/dashboard)  
**Submission:** [S-757](https://code4rena.com/audits/2026-02-jupiter-lend/submissions/S-757)  
**Type:** Code4rena QA Report  
**Status:** Public QA Research  
**Researcher:** 0xBerberSec

## Summary

During review of the Vault liquidation and absorb flow, a branch explicitly marked as `Closed` was found to be assignable later as `current_branch_id` through `update_state_at_liq_end`, without validation of its lifecycle status.

No immediate economic exploit or fund loss was identified. The issue is a defensive programming gap that weakens internal branch lifecycle invariants and reduces confidence in branch state transitions.

This finding relates directly to the audit scope concern:

```text
Branch state should be maintained at all places required, like closed and merged.
```

## Technical Details

During `absorb()`, a branch is marked as closed:

```rust
branch_to_update.set_state_after_absorb(&branch_data.data);
```

The called function transitions the branch lifecycle state to `Closed`:

```rust
pub fn set_state_after_absorb(&mut self, branch_data: &BranchState) {
    self.set_status_as_closed();
    self.minima_tick = branch_data.minima_tick;
    self.minima_tick_partials = branch_data.minima_tick_partials;
    self.debt_liquidity = branch_data.debt_liquidity;
    self.debt_factor = branch_data.debt_factor;
    self.connected_branch_id = branch_data.connected_branch_id;
    self.connected_minima_tick = branch_data.connected_minima_tick;
}
```

Later in the same flow, the vault state is updated:

```rust
vault_state.update_state_at_liq_end(branch_data.minima_tick, branch_data.id)?;
```

The update function assigns the provided branch as active without checking whether it is already closed:

```rust
pub fn update_state_at_liq_end(&mut self, tick: i32, branch_id: u32) -> Result<()> {
    self.set_branch_liquidated();
    self.topmost_tick = tick;
    self.current_branch_id = branch_id;
    Ok(())
}
```

As a result, a branch that has just been marked `Closed` can be reassigned as the active `current_branch_id`.

## Impact

Confirmed impact:

- No confirmed fund loss
- No confirmed liquidation bypass
- No demonstrated debt accounting mismatch

Security significance:

- Weakens branch lifecycle invariants
- Reduces defensive guarantees around liquidation state transitions
- Increases future maintenance risk in a complex branch, absorb, merge, and debt-factor system

This is categorized here as QA research rather than a validated finding.

## Recommendation

Add explicit lifecycle validation before assigning a branch as active.

For example, before updating `current_branch_id`, enforce that the target branch is not closed:

```rust
require!(!branch.is_closed(), ErrorCodes::InvalidBranchState);
```

Alternatively, lifecycle guarantees can be enforced at the absorb transition layer so that a closed branch cannot later be used as the active branch.

## Conclusion

The implementation allows a branch marked as `Closed` to be reassigned as active without explicit lifecycle validation.

Although no immediate economic exploit was identified, this weakens internal lifecycle invariants and directly matches the audit's stated focus on correct branch state management.

## Links

- [Code4rena submission S-757](https://code4rena.com/audits/2026-02-jupiter-lend/submissions/S-757)
- [Jupiter Lend audit dashboard](https://code4rena.com/audits/2026-02-jupiter-lend/dashboard)

[Back to portfolio](../../README.md)
