# Protocol Investigation Case Studies

This section contains authorized real-protocol case studies and separate generic methodological overviews. Neither category is a client audit or security certification.

## Real protocol case studies

- [Mezo Pool — Reserve Invariants and Bounded Fuzzing](mezo-pool-fuzz.md)
- [Mezo UniversalRouter — Authorization, Deadlines and Execution Controls](mezo-universal-router.md)
- [Mezo overview](mezo-independent-research.md) and [minimal reproduction harness](mezo-reproduction/README.md)

Mezo experiments use a historical local fork and bounded assertions. No confirmed exploitable Mezo finding is claimed. The harness documents conditional quote checks, low-level rejection checks and their limits.

## Generic methodological overviews

The pages below omit protocol-specific vulnerability details and private execution artifacts. They describe reasoning rather than independently inspectable exploit proofs.

| Overview | Research question | Method |
| --- | --- | --- |
| [Accounting models](accounting-models.md) | Are apparently different value measurements answering the same question? | Define units and accounting identities, then measure states |
| [Reward lifecycle](reward-lifecycle.md) | Can state transitions consume the same entitlement twice? | Model states, reverse action ordering, check deltas |
| [Temporal state](temporal-state.md) | Which timestamp or state snapshot controls execution? | Compare clocks, permission boundaries and execution ordering |
| [Economic counter-tests](economic-counter-tests.md) | Where does an apparent gain come from? | Track all contributors, compare controls and qualify net impact |

The dossiers underlying these generic overviews remain withheld until their disclosure basis is established. They must not be confused with the selected authorized Mezo case studies above.

A separate [original educational lab](../educational-demos/refund-escrow/README.md) makes the workflow executable without releasing any of the underlying private cases.

[Audit methodology](../methodology/README.md) · [Evidence policy](../docs/SEVERITY_AND_CLAIMS.md) · [Portfolio](../README.md)
