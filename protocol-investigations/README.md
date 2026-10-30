# Protocol Investigation Case Studies

The pages below are **generic methodological overviews**, based on themes in my research. They omit protocol-specific vulnerability details, source files, parameters and private execution artifacts. They are not published case reports or independently inspectable PoCs.

| Overview | Research question | Method |
| --- | --- | --- |
| [Accounting models](accounting-models.md) | Are apparently different value measurements answering the same question? | Define units and accounting identities, then measure states |
| [Reward lifecycle](reward-lifecycle.md) | Can state transitions consume the same entitlement twice? | Model states, reverse action ordering, check deltas |
| [Temporal state](temporal-state.md) | Which timestamp or state snapshot controls execution? | Compare clocks, permission boundaries and execution ordering |
| [Economic counter-tests](economic-counter-tests.md) | Where does an apparent gain come from? | Track all contributors, compare controls and qualify net impact |

The underlying technical dossiers remain withheld until their disclosure basis is established. No protocol-specific test counts, success claims or private references are introduced here.

A separate [original educational lab](../educational-demos/refund-escrow/README.md) makes the workflow executable without releasing any of the underlying private cases.

[Audit methodology](../methodology/README.md) · [Evidence policy](../docs/SEVERITY_AND_CLAIMS.md) · [Portfolio](../README.md)
