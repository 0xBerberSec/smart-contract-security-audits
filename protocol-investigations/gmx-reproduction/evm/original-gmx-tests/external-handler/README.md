# ExternalHandler — external-call boundary and refunds

Executable file: [`../external-handler-safe-controls.spec.ts`](../external-handler-safe-controls.spec.ts)

The adaptation uses the real GMX `deployFixture`, `ExchangeRouter` and
`ExternalHandler` contracts. It preserves two original control properties:

1. When `refundTokens` contains the input token, the handler's balance is zero
   and the originating user's balance is restored.
2. If a later `multicall` item reverts, the earlier transfer into the handler is
   rolled back and the handler's balance remains zero.

Original source references:

- `contracts/router/ExchangeRouter.sol:104-120`
- `contracts/external/ExternalHandler.sol:32-67`
- `test/guardian/testExternalHandlerValidation.ts:24-36,142-155`

The public adaptation returned **2 passing, exit 0** with `--no-compile` on the
identified clean-build artifacts. The complete seven-test file remains private
because its residual-output and partial-fill paths are research-specific.
