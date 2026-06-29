# GMX Synthetics - Disabled Integration Relay Bypass

[Back to portfolio](../../README.md)

## Report

**Title:** Disabled Integration Relay Bypass Enables UI Fee / Affiliate Fee Siphon  
**Protocol:** GMX Synthetics  
**Type:** Independent Security Research  
**Status:** Public independent research  
**Researcher:** 0xBerberSec

## Overview

This research reviews the GMX Synthetics subaccount relay path and a failure mode around disabled integrations.

The relay path validates the `integrationId` already stored for an `(account, subaccount)` pair, but a fresh relay `subaccountApproval` also carries an `integrationId`. In the reviewed flow, that signed `integrationId` is not persisted or enforced before the relay action proceeds.

As a result, an integration that has been disabled can still onboard or rearm a subaccount through fresh relay approvals. Once the subaccount remains usable, orders submitted for the account can direct UI fee attribution and referral attribution to attacker-controlled receivers.

This report is independent research and is not associated with any official contest ruling.

## Root Cause

`SubaccountRouterUtils.handleSubaccountAction()` validates the stored integration state before applying the fresh relay approval:

```text
validateIntegrationId(account, subaccount)
_handleSubaccountApproval(...)
handleSubaccountAction(...)
```

`SubaccountUtils.validateIntegrationId()` reads only the stored value:

```solidity
bytes32 integrationId = dataStore.getBytes32(
    Keys.subaccountIntegrationIdKey(account, subaccount)
);
```

The fresh signed value from `subaccountApproval.integrationId` is present, but `SubaccountUtils.handleSubaccountApproval()` only updates approval metadata such as:

- allowed action count
- expiry
- subaccount membership

It does not persist or enforce the signed `integrationId`.

This creates a mismatch between the integration identifier used by the relay approval and the integration identifier checked by the disabled-integration guard.

## Security Invariant

```text
Disabled integrations must never be able to onboard or rearm subaccounts through the relay path.
```

The invariant is broken because the disabled-integration guard checks stale stored state while the fresh signed relay approval can carry a disabled `integrationId`. If the stored value remains unset or unrelated, the disabled integration is not blocked at the point where the new approval is accepted.

## Attack Flow

```text
Integration active
        |
        v
Subaccount approval flow exists
        |
        v
Integration disabled
        |
        v
Fresh relay approval includes disabled integrationId
        |
        v
Relay checks old stored integrationId
        |
        v
Relay authorization succeeds
        |
        v
Subaccount can submit orders for the account
        |
        v
Attacker-controlled uiFeeReceiver / referral code is used
        |
        v
UI fees and affiliate rewards become claimable by attacker-controlled addresses
```

## Impact

The demonstrated impact is limited to the tested fee-routing paths:

- disabled integration relay onboarding succeeds when it should be blocked;
- subaccount access can be rearmed after the original action quota is exhausted;
- repeated subaccount orders can accrue UI fees to an attacker-controlled `uiFeeReceiver`;
- affiliate reward attribution can also be redirected through the same compromised subaccount foothold.

The PoC demonstrates repeated siphoning of protocol-funded UI fees and affiliate rewards after bypassing the intended integration kill-switch.

This report does not claim:

- total protocol drain;
- broader protocol control compromise;
- impact without user signatures;
- external contest ruling.

## Limitations

This research depends on a threat model where a user signs fresh relay approvals after an integration has been disabled, and where the subaccount or integration path is controlled by an attacker.

The report does not show arbitrary takeover of unrelated accounts. It also does not claim that all subaccount integrations are unsafe. The demonstrated issue is narrower: the disabled-integration guard does not bind to the fresh signed `integrationId`, allowing the tested relay onboarding and rearming path to continue despite the intended kill-switch.

The UI fee and affiliate reward impacts are demonstrated in the local GMX Synthetics test environment. They are presented as reproducible security research, not as an externally confirmed production incident.

## Proof of Concept

The PoC uses the GMX Synthetics test suite and focuses on four behaviors:

- disabled-integration relay onboarding still succeeds;
- attacker-controlled referral attribution can receive affiliate rewards;
- attacker-controlled `uiFeeReceiver` can accrue and claim UI fees;
- a fresh relay approval can rearm the subaccount after the initial quota is exhausted.

See [PoC.md](./PoC.md).

## Remediation Direction

The relay approval flow should bind the signed `subaccountApproval.integrationId` to the disabled-integration check.

Possible approaches:

- persist the fresh signed `integrationId` atomically before allowing the relay action;
- reject a non-zero signed `integrationId` unless it matches the stored integration state;
- validate the signed `integrationId` directly when processing fresh relay approvals;
- restrict subaccount-created orders from setting arbitrary third-party `uiFeeReceiver` values unless explicitly authorized.

## Links

- [PoC](./PoC.md)
- [Back to portfolio](../../README.md)
