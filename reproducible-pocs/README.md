# Reproducible PoCs and Educational Demonstrations

## Available now: original educational lab

[Refund Escrow](../educational-demos/refund-escrow/README.md) includes intentionally vulnerable and corrected source, a security invariant, targeted tests, economic controls and regression checks. It was created with Codex assistance on 30 October 2026 to make the research workflow inspectable. The flaw is a known reentrancy class introduced deliberately, not a discovery on a third-party protocol.

The [actual local run](../educational-demos/refund-escrow/EVIDENCE.md) records 13 passing tests and a separate intentionally failing safety assertion. Source, configuration, trace, seed, hashes and a standard-library Python replay runner are included. No RPC, keys or third-party Solidity libraries are required.

## Third-party protocol research

No runnable third-party protocol exploit is supplied. Independent research code and historical artifacts remain private until source provenance and disclosure rights are established. A local test or an older public narrative is not external acceptance or permission to disclose.

An authorized future reproduction should preserve the exact source revision and patch, dependencies, actors, fixture changes, assertions, controls, result, economic limits and disclosure basis. Historical results stay separate from fresh executions.

[Testing approach](../research-tools-testing/testing.md) · [Evidence register](../docs/EVIDENCE.md) · [Portfolio](../README.md)
