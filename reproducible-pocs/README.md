# Reproducible PoCs and Educational Demonstrations

## Available now: original educational lab

[Refund Escrow](../educational-demos/refund-escrow/README.md) includes intentionally vulnerable and corrected source, a security invariant, targeted tests, economic controls and regression checks. It was created with Codex assistance on 30 October 2026 to make the research workflow inspectable. The flaw is a known reentrancy class introduced deliberately, not a discovery on a third-party protocol.

The [actual local run](../educational-demos/refund-escrow/EVIDENCE.md) records 13 passing tests and a separate intentionally failing safety assertion. Source, configuration, trace, seed, hashes and a standard-library Python replay runner are included. No RPC, keys or third-party Solidity libraries are required.

## Third-party protocol research

No runnable third-party protocol exploit is supplied. Selected authorized Mezo boundary tests are available below; other independent research code and historical artifacts remain private unless source provenance and disclosure rights support release. A local test or an older public narrative is not external acceptance or permission to disclose.

An authorized future reproduction should preserve the exact source revision and patch, dependencies, actors, fixture changes, assertions, controls, result, economic limits and disclosure basis. Historical results stay separate from fresh executions.

## Mezo — Reproducible Real-Protocol Boundary Tests

The [Mezo harness](../protocol-investigations/mezo-reproduction/README.md) supplies selected Pool and Router test sources, Foundry configuration, a local-fork runner and a [sanitized reference record](../protocol-investigations/mezo-reproduction/evidence/reference-run.json). It uses historical block 12674080, chain ID 31612 and the documented toolchain. Reproduction requires a compatible historical Mezo RPC; no wallet or public transaction is required.

The recorded results are one Pool quote fuzz test with 1,024 runs, two Pool accounting tests and four Router boundary tests. The fuzzer reads fixed state; Router assertions primarily check low-level rejection without identifying the exact internal cause. These are bounded experiments, not an exhaustive security proof or a published third-party exploit. No confirmed exploitable Mezo vulnerability is claimed.

The existing [Refund Escrow CI](../educational-demos/refund-escrow/CI.md) covers the educational lab only and does not constitute hosted Mezo CI. Officially validated outcomes remain in the separate [validated findings](../validated-findings/README.md) section. Raw private research transcripts are not treated as public execution evidence.

[Testing approach](../research-tools-testing/testing.md) · [Evidence register](../docs/EVIDENCE.md) · [Portfolio](../README.md)

### Mezo correction — 1 November 2026

The old two UniversalRouter London PASS labels came from `NotActivated` and do not validate their intended functional checks. The [separate complementary harness](../protocol-investigations/mezo-reproduction/supplemental/README.md) supplies non-vacuous quote coverage, exact errors and a positive empty execution under a qualified Cancun profile, plus historical RPC corroboration for four selected calls. It is not a published exploit or a full stateful invariant campaign.
