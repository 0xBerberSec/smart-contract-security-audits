# Methodology

[Back to index](./INDEX.md)

This repository is intentionally selective. Reports are published only when the root cause, impact path, and proof of concept can be explained without overstating the result.

## Review Process

Research generally follows this sequence:

1. Read protocol documentation and scoped contracts.
2. Map trust boundaries, privileged roles, user-controlled inputs, and asynchronous state transitions.
3. Identify security invariants before looking for exploit paths.
4. Trace the full call path from external entry point to final state change.
5. Build a minimal PoC that demonstrates the exact claim.
6. Check the strongest rebuttal before publication.
7. Classify the work conservatively.

## Focus Areas

- DeFi lending and liquidation flows
- Cross-chain messaging and endpoint configuration
- Subaccount, relay, and delegated-action systems
- Runtime validation mismatches
- State-machine and lifecycle invariants
- Fee accounting and attribution paths
- Configuration and permission boundary failures

## Publication Standard

A report is publishable only when it clearly states:

- what is proven;
- what is not proven;
- which assumptions are required;
- how to reproduce the behavior;
- why the issue matters under a realistic threat model.

If the strongest version of a report still depends on weak assumptions, unclear impact, or unsupported severity claims, it remains private.

## Non-Goals

This portfolio does not publish raw scanner output, unconfirmed hypotheses, dead-end research notes, or findings that require speculative impact to look important.

[Back to index](./INDEX.md)
