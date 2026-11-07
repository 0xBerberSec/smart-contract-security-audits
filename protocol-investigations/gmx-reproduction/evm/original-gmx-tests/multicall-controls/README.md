# Multicall and counter-testing

The rollback control is included in
[`../external-handler-safe-controls.spec.ts`](../external-handler-safe-controls.spec.ts).
It reaches GMX's `PayableMulticall` delegatecall loop and verifies the post-revert
balance rather than treating a revert alone as proof.

The complete partial-fill and multi-token variants remain in the private
original test file. Those variants are the strongest counter-tests, but their
exact residual-token sequences are not cleared for publication.

Source references:

- `contracts/utils/PayableMulticall.sol:14-31`
- `test/guardian/testExternalHandlerValidation.ts:68-189`
