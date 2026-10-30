# Research Tools & Testing

My strongest recurring themes are DeFi accounting, authorization boundaries and state transitions. The table describes targeted work reflected in my research artifacts. It does not claim equal expertise in every runtime or provide public execution evidence for withheld studies.

| Area | Research use | Qualification |
| --- | --- | --- |
| Solidity / EVM | Contract logic, token accounting, roles, reverts and storage effects | Focused investigations, not exhaustive EVM expertise |
| Foundry | Solidity test harnesses, concrete assertions and control cases | A source file alone is not proof of execution |
| Hardhat / TypeScript / ethers | Protocol fixtures, fork resets, local impersonation, balances and events | Impersonated privileges do not become attacker permissions |
| Rust / Soroban | Targeted runtime and contract tests | No general cross-chain validation claim |
| Solana / Anchor | Model-level and local integration experiments | Mock/configuration limits remain relevant |
| Cairo tooling | Targeted lifecycle and conservation tests | Local dependency substitutions require qualification |
| Python / PowerShell / JavaScript | Data collection, provenance, corpus indexing and evidence preparation | Target-specific automation; no measured productivity claim |
| Traces and state differences | Relating execution paths to observed writes and accounting changes | Partial reconstruction stays explicitly partial |
| Invariant / fuzz harness work | Extending actions and accounting assertions | Short smoke runs without useful corpus growth are not deep coverage |
| Codex | Assistance with navigation, reasoning and test drafts | Generated output requires verification |

## Inspect an executed example

The [Refund Escrow lab](../educational-demos/refund-escrow/README.md) supplies source, a callback harness, counter-tests, a correction, fuzz inputs, a stateful actor model and actual run artifacts. It provides concrete evidence of local Foundry test construction and interpretation on a small synthetic system. Its limited scope is distinct from experience with complex third-party protocols.

## Scope of experience

I have investigated vault, LP, reward, lending and derivatives accounting questions. This portfolio does not claim completed client audits across all researched protocols, a history of private mandates, formal verification expertise or comprehensive fuzzing coverage.

[Testing approach](testing.md) · [Investigation overviews](../protocol-investigations/README.md) · [Evidence register](../docs/EVIDENCE.md) · [Portfolio](../README.md)
