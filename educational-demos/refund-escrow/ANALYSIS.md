# Hypothesis, invariant, controls and correction

## Protocol and threat model

The protocol accepts ETH deposits and records one refundable credit per depositor. Its only economic operation is refunding that recorded amount to the caller. Funds are pooled. An attacker may deposit through a contract and execute code when receiving ETH; no administrative role or existing user signature is assumed. All values in this demonstration are synthetic local-EVM balances supplied by the test fixture.

## Security invariant

For the explicitly tracked actor set S, define:

```text
liabilities(S) = sum(escrow.credits(actor) for actor in S)
escrow ETH balance >= liabilities(S)
```

Each credited amount must be consumed at most once. The stateful handler constrains all depositors to Alice, Bob and the callback receiver, making S complete for that experiment. In an open production system, summing an incomplete actor list would not establish system solvency. Direct forced ETH transfers can produce harmless surplus, which is why the safety condition is >= rather than universal equality.

## Hypothesis and root cause

The vulnerable `refund()` reads an entitlement, sends ETH to an external recipient and only then sets the credit to zero. A callback can re-enter while that entitlement remains visible. The outer frame later stores zero again, without undoing the second payment. This is a deliberately introduced, known checks-effects-interactions violation.

The proposed fix moves the credit-consumption write before the external call. A nested refund then observes zero and rejects. A transfer failure reverts the entire call, restoring the original credit; this rollback is checked explicitly.

## Deterministic experiment and source of impact

| Quantity | Vulnerable version | Fixed version |
| --- | ---: | ---: |
| Other depositor's contribution | 4 synthetic ETH | 4 synthetic ETH |
| Attacker's contribution | 1 synthetic ETH | 1 synthetic ETH |
| Total paid to callback receiver | 2 synthetic ETH | 1 synthetic ETH |
| Receiver receipt minus contribution, before gas | +1 synthetic ETH | 0 |
| Remaining escrow ETH | 3 synthetic ETH | 4 synthetic ETH |
| Remaining victim credit | 4 synthetic ETH | 4 synthetic ETH |
| Liability deficit | 1 synthetic ETH | 0 |

The extra payment is funded by the other depositor's backing. No ETH is created by the protocol. This is a local accounting-loss demonstration; gas is not valued, so +1 is not a claim of net profit on any production network.

## Test-to-claim map

| Test / group | What it checks | What it does not establish |
| --- | --- | --- |
| `test_vulnerableCallbackConsumesSameCreditTwice` | A nested call executes and leaves a measured deficit | Exploitability of a third-party protocol |
| `test_counter_noCallbackKeepsVulnerablePoolSolvent` | Removing the callback removes this observed extra payout | Overall safety of the vulnerable version |
| `test_counter_noOtherDepositorMeansNoExtractableSurplus` | This bounded receiver returns its own deposit when the pool has no extra backing | Every possible call strategy in an empty pool |
| Normal EOA refund and uncredited-caller controls | Ordinary refunds work; a caller without credit cannot withdraw | Callbacks are harmless |
| `test_fixedRejectsNestedRefundButPaysOriginalCredit` | The callback is attempted and rejected; the legitimate refund still completes | A generic reentrancy-proof protocol |
| `test_fixedRejectingReceiverRestoresCreditOnRevert` | Failed delivery restores both credit and backing atomically | Guaranteed delivery to recipients that refuse ETH |
| Repeated refund, empty deposit and victim exit tests | Relevant boundaries and continued victim exit on the fix | Liveness under all external conditions |
| Two fuzz tests | Bounded positive amounts reproduce the local contrast between versions | Exhaustive input or state-space coverage |
| `invariant_balanceCoversCredits` | The fixed version covers tracked liabilities over bounded handler sequences | All actor models, callback strategies or future modifications |
| Separate `test_invariantBalanceCoversCreditsAfterRefund` | The safety property itself fails on the vulnerable version, with a trace | A failed compiler/setup being mistaken for a vulnerability |

## Scope of the correction

Checks-effects-interactions repairs the demonstrated duplicate consumption. The tests include rejection of a second claim and rollback on an unsuccessful ETH delivery. They do not replace a production review: there are no fees, time locks, tokens, ownership roles, oracles, upgrades or deployment scripts. Other callback behaviors and cross-function interactions would require additional hypotheses and tests.

[Executed results](EVIDENCE.md) · [Source](src/RefundEscrow.sol) · [Test suite](test/RefundEscrow.t.sol) · [README](README.md)
