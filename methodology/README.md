# Audit Methodology

**Value Flow → Security Invariants → Targeted Tests → Counter-Tests → Economic Impact Qualification**

My starting point is the protocol’s economic model. I map how assets and authority move, turn assumptions into testable properties, and use the result to refine or reject a hypothesis. The workflow is iterative: missing source, an unexplained measurement or a stronger counterargument can send the investigation back to its assumptions.

## 1. Value flow

I establish the scope and source revision, then map entry points, dependencies, privileged actors and external calls. I trace deposits, withdrawals, claims and other value-moving paths through the contracts implementing the relevant logic.

Accounting views need explicit units and meaning: a stored balance, a current estimate and a realizable withdrawal amount may answer different questions. [Accounting overview](../protocol-investigations/accounting-models.md).

## 2. Security invariants

Define what must hold across the relevant states and actors. Examples of properties to examine include value conservation, entitlement consumed at most once, agreement between global and individual accounting, and consistent application of authorization policy.

An invariant guides the review, but only a concrete setup and executable assertion can test it. I prioritize hypotheses by impact, reachability and uncertainty rather than treating every unusual pattern as a vulnerability.

## 3. Targeted tests

I use the smallest experiment that can distinguish the hypothesis from expected behavior. Where practical I reuse the protocol's fixtures, documenting actors, dependencies, mocks and privileged setup. For local forks, I record the block and the actions exercised.

I inspect the assertions, before-and-after values, traces and state transitions. I check that the test reached the intended property and distinguish a passing safety assertion from a test that passes because an anomaly or expected failure occurred. [Testing approach](../research-tools-testing/testing.md).

## 4. Counter-tests

I test the strongest alternative explanations, including supported execution flows, expected ordering, remaining permission checks and prior findings. Trusted-role behavior and design choices are not treated as attacker-controlled paths.

I record why an idea was rejected and what new evidence would justify reopening it. Negative results are useful when their scope and assumptions are explicit. [Lifecycle overview](../protocol-investigations/reward-lifecycle.md) · [Temporal-state overview](../protocol-investigations/temporal-state.md).

## 5. Economic impact qualification

I identify who can act, who loses, who gains and where the value originates. I distinguish an observed local primitive from a complete exploitation path and record unpriced costs, synthetic state, missing steps and uncertain production configuration.

A demonstrated behavior can remain an observation when its impact is not established. Test success and external acceptance are separate questions. [Economic controls](../protocol-investigations/economic-counter-tests.md) · [Finding validation](validation.md).

## Audit-OS and research assistance

Audit-OS is my working environment for scope notes, architecture maps, invariants, hypotheses, tests, traces and conclusions. Target-specific scripts help collect data, compare sources and prepare evidence; this is not a claim of a unified autonomous audit engine.

I use **Codex as research assistance** for code navigation, analysis and drafting tests. Suggestions and generated code require source review and technical verification. An AI explanation is not a test result, a generated PoC is not a successful execution, and neither establishes independent validation.

## Executable example

The [original Refund Escrow lab](../educational-demos/refund-escrow/README.md) maps this workflow to code: a backing invariant, callback hypothesis, targeted experiment, control cases, economic attribution and correction. Its [actual execution record](../educational-demos/refund-escrow/EVIDENCE.md) includes a passing default suite, a deliberately failing vulnerable safety assertion and a bounded fixed-version invariant campaign. The [research-process page](../educational-demos/refund-escrow/RESEARCH_PROCESS.md) states Codex's contribution and the verification boundary. This is a teaching artifact, not an audit finding on a third-party protocol.

## Public evidence

The [Injective result](../validated-findings/injective-peggy-bridge/README.md) is the externally verified outcome presented here. The generic investigation overviews explain methods without disclosing their underlying private technical evidence; they do not imply those techniques produced the Injective finding.

Selected [Mezo case studies](../protocol-investigations/mezo-independent-research.md), Solidity test sources, runners, reproduction instructions and documented results are public. Start with the [current complementary evidence](../protocol-investigations/mezo-reproduction/supplemental/README.md); the [historical harness](../protocol-investigations/mezo-reproduction/README.md) preserves the initial experiments. These are bounded independent protocol experiments, not a confirmed third-party exploit or a Mezo security certification. The educational lab above remains a separate synthetic example.

Other research dossiers, findings not authorized for disclosure, confidential evidence and experiments not cleared for publication remain private. No runnable third-party protocol exploit is supplied.

[Research workflow](workflow.md) · [Finding validation](validation.md) · [Research tools](../research-tools-testing/README.md) · [Portfolio](../README.md)
