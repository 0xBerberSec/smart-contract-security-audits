# Proof of Concept

[Back to GMX report](./README.md)

## Overview

The PoC demonstrates that a disabled integration can still use fresh relay approvals to onboard or rearm a subaccount because the relay path validates the stored integration identifier rather than the fresh `integrationId` included in `subaccountApproval`.

After that bypass, repeated subaccount orders can route UI fees and affiliate rewards to attacker-controlled receivers.

## Environment

- Protocol: GMX Synthetics
- Component: Subaccount relay / fee routing
- Test file: `test/router/relay/SubaccountGelatoRelayRouter.ts`
- Test runner: Hardhat

## Steps

1. Mark an `integrationId` as disabled in `DataStore`.
2. Submit a fresh relay `subaccountApproval` containing the disabled `integrationId`.
3. Observe that the relay approval succeeds because the stored `(account, subaccount)` integration value remains unset.
4. Use the authorized subaccount to submit repeated orders for the account.
5. Set `uiFeeReceiver` to an attacker-controlled address.
6. Execute the orders so UI fees accrue to the attacker-controlled receiver.
7. Claim the accrued UI fees.
8. Repeat with a fresh disabled-integration approval to rearm the action quota.

## Expected Result

Once an integration is disabled, fresh relay approvals tied to that integration should not onboard or rearm subaccounts.

The disabled-integration guard should reject the relay path before the subaccount can be used for further order activity.

## Observed Result

The fresh relay approval succeeds even though it includes a disabled `integrationId`.

The tests show:

- the stored integration id remains unset;
- subaccount order creation remains available;
- attacker-controlled UI fee attribution accrues claimable balances;
- affiliate reward attribution can also be redirected;
- a fresh approval can rearm the subaccount after the original quota is exhausted.

The focused test run completed successfully:

```text
4 passing
```

Test names:

```text
diverts affiliate rewards to an attacker-controlled referral code after disabled integration relay onboarding
can siphon meaningful affiliate rewards from repeated victim trading after disabled integration relay onboarding
can siphon meaningful ui fees from repeated direct subaccount orders after disabled integration relay onboarding
can rearm disabled-integration access and continue siphoning ui fees after the original subaccount quota is exhausted
```

## Command

```powershell
Set-Location 'C:\Users\zidan\Desktop\audit-os\targets\gmx-synthetics'
& 'C:\Users\zidan\Desktop\contracts\.tools\node\corepack.cmd' yarn hardhat test test/router/relay/SubaccountGelatoRelayRouter.ts --grep "can siphon meaningful ui fees from repeated direct subaccount orders after disabled integration relay onboarding|can rearm disabled-integration access and continue siphoning ui fees after the original subaccount quota is exhausted|diverts affiliate rewards to an attacker-controlled referral code after disabled integration relay onboarding|can siphon meaningful affiliate rewards from repeated victim trading after disabled integration relay onboarding"
```

## Conclusion

The PoC demonstrates a cross-module issue in the relay approval and fee attribution flow:

- the disabled-integration guard does not bind to the fresh signed integration id;
- subaccount access can remain usable after the integration is disabled;
- fee attribution can be routed to attacker-controlled receivers;
- the quota can be refreshed through another fresh relay approval.

This is presented as independent security research and is not associated with any official contest ruling.

[Back to GMX report](./README.md)
