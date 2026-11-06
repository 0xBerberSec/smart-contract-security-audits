# Evidence Register

Updated on **1 November 2026**; historical results retain their original dates. Educational evidence, independent protocol experiments and external validation are separate categories.

| Public material | Evidence class | What can be verified | Limit |
| --- | --- | --- | --- |
| [Injective Peggy Bridge](../validated-findings/injective-peggy-bridge/README.md) | Officially validated outcome | Code4rena profile and competition results: 1 Medium, 21st place | Original technical submission and reproduction are not supplied |
| [Refund Escrow lab](../educational-demos/refund-escrow/README.md) | New educational demonstration, locally executed | Source, 13-test passing suite, deliberate counterexample failure and fixed-contract invariant campaign | Synthetic protocol and bounded actor model; not third-party research or external validation |
| [GMX — cross-architecture research](../protocol-investigations/gmx-cross-architecture-security-research.md) | Independent protocol research | Three architectures; 36 historical investigation records (not 36 vulnerabilities); 431 Solidity files compiled; 15 targeted local Hardhat tests (4 Relay, 7 ExternalHandler, 4 OrderVault); one fresh Rust Borrowing model test | No fresh Anchor reproduction; local fixture/model evidence only; private evidence and PoCs are not supplied; no confirmed permissionless GMX economic finding |
| [Mezo research](../protocol-investigations/mezo-independent-research.md) | Independent real-protocol research with local fork reproductions | Recorded Pool quote fuzz: 1,024 runs PASS; Pool accounting: 2 PASS; Router deadline/path boundaries: 2 PASS; UniversalRouter historical London: 2 PASS caused by `NotActivated`, not intended checks; local fork block 12674080, chain ID 31612; no public transactions. [Harness and reproduction record](../protocol-investigations/mezo-reproduction/README.md) | Bounded assertions only; Router low-level failure does not prove its internal cause; no permissionless economic exploit or security certification; complete research records remain private |
| [Audit methodology](../methodology/README.md) | Research practice with a linked executable teaching example | Source-to-invariant-to-test-to-control-to-fix reasoning in the lab | Not independent certification or evidence of how the Injective finding was obtained |
| [Generic investigation overviews](../protocol-investigations/README.md#generic-methodological-overviews) | Generic methodological descriptions, separate from the Mezo studies | The reasoning described on each page | Underlying private artifacts are not available for public technical verification |
| [Research tools](../research-tools-testing/README.md) | Bounded experience plus actual educational execution | Foundry harness, fuzzing, stateful handler and replay evidence in the lab | Broader tools/private-study experience is not established by the teaching example alone |

LayerZero and Jupiter individual platform rulings have not been verified. Their aggregate reports do not establish individual acceptance. Existing legacy links are retained as compatibility records, not additional validated outcomes.

The research inventory contains hypotheses, observations, rejected paths and source-limited investigations. It is not a vulnerability count and is not a claim of completed client audits. Private inventories, source paths and technical evidence are excluded from this selection.

[Evidence classes](SEVERITY_AND_CLAIMS.md) · [Disclosure](DISCLOSURE.md) · [Portfolio](../README.md)

## Mezo correction and complementary evidence — 1 November 2026

The old UniversalRouter London PASS labels do not verify functional controls: both calls hit `NotActivated`. See the [dated correction](../protocol-investigations/mezo-universal-router.md#technical-correction--1-november-2026).

The separate [complementary record](../protocol-investigations/mezo-reproduction/supplemental/evidence/verified-upgrade.json) records 60 successful grid quotes (48 positive outputs), six directions, 1,024 material-input fuzz cases with six asserted calls per case, and exact Router/UniversalRouter controls. Nine local functions PASS under Cancun (one logger); London gives seven PASS/two FAIL. Four selected UniversalRouter outcomes were corroborated by separate historical read-only RPC simulations using the same provider; this is not independent multi-provider validation. Local opcode compatibility is not full production-EVM equivalence; no permissionless economic exploit or certification is claimed. Offline runner tests are a separate category, not hosted fork CI.
