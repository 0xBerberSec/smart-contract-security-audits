# Evidence Register

Updated on **31 October 2026**. Educational evidence, independent protocol experiments and external validation are separate categories.

| Public material | Evidence class | What can be verified | Limit |
| --- | --- | --- | --- |
| [Injective Peggy Bridge](../validated-findings/injective-peggy-bridge/README.md) | Officially validated outcome | Code4rena profile and competition results: 1 Medium, 21st place | Original technical submission and reproduction are not supplied |
| [Refund Escrow lab](../educational-demos/refund-escrow/README.md) | New educational demonstration, locally executed | Source, 13-test passing suite, deliberate counterexample failure and fixed-contract invariant campaign | Synthetic protocol and bounded actor model; not third-party research or external validation |
| [Mezo research](../protocol-investigations/mezo-independent-research.md) | Independent real-protocol research with local fork reproductions | Recorded Pool quote fuzz: 1,024 runs PASS; Pool accounting: 2 PASS; Router deadline/path boundaries: 2 PASS; UniversalRouter boundaries: 2 PASS; local fork block 12674080, chain ID 31612; no public transactions. [Harness and reproduction record](../protocol-investigations/mezo-reproduction/README.md) | Bounded assertions only; Router low-level failure does not prove its internal cause; no permissionless economic exploit or security certification; complete research records remain private |
| [Audit methodology](../methodology/README.md) | Research practice with a linked executable teaching example | Source-to-invariant-to-test-to-control-to-fix reasoning in the lab | Not independent certification or evidence of how the Injective finding was obtained |
| [Generic investigation overviews](../protocol-investigations/README.md#generic-methodological-overviews) | Generic methodological descriptions, separate from the Mezo studies | The reasoning described on each page | Underlying private artifacts are not available for public technical verification |
| [Research tools](../research-tools-testing/README.md) | Bounded experience plus actual educational execution | Foundry harness, fuzzing, stateful handler and replay evidence in the lab | Broader tools/private-study experience is not established by the teaching example alone |

LayerZero and Jupiter individual platform rulings have not been verified. Their aggregate reports do not establish individual acceptance. Existing legacy links are retained as compatibility records, not additional validated outcomes.

The research inventory contains hypotheses, observations, rejected paths and source-limited investigations. It is not a vulnerability count and is not a claim of completed client audits. Private inventories, source paths and technical evidence are excluded from this selection.

[Evidence classes](SEVERITY_AND_CLAIMS.md) · [Disclosure](DISCLOSURE.md) · [Portfolio](../README.md)
