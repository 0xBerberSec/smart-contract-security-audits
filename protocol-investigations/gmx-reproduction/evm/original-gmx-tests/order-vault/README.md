# OrderVault — physical versus internal accounting

Executable file: [`../vault-accounting-observations.spec.ts`](../vault-accounting-observations.spec.ts)

The adaptation deploys the real GMX fixture and sends USDC into `OrderVault`
and `DepositVault`. It asserts the accounting distinction directly:

```text
token.balanceOf(vault) == strandedAmount
vault.tokenBalances(token) == 0
```

It also checks that the unrelated user has no balance. This preserves the
initial accounting observation from the original tests while omitting the
subsequent request/capture continuation.

Original source references:

- `test/guardian/testVaultResiduals.ts:37-48`
- `test/guardian/testVaultResiduals.ts:106-117`

The public adaptation returned **2 passing, exit 0** with `--no-compile`. The
complete increase, deposit, GLV and withdrawal sequences remain private pending
disclosure approval.
