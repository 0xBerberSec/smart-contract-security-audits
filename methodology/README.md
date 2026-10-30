# Audit Methodology

**Value Flow → Security Invariants → Targeted Tests → Counter-Tests → Economic Impact Qualification**

My starting point is the protocol’s economic model. I map how assets and authority move, turn assumptions into testable properties, and use the result to refine or reject a hypothesis. The workflow is iterative: missing source, an unexplained measurement or a stronger counterargument can send the investigation back to its assumptions.

## 1. Value flow

Establish the scope and source revision. Map entry points, contracts, dependencies, privileged actors and external calls. Trace deposits, withdrawals, claims and other value-moving paths through the components that implement the business logic.

Accounting views need explicit units and meaning: a stored balance, a current estimate and a realizable withdrawal amount may answer different questions. [Accounting overview](../protocol-investigations/accounting-models.md).

## 2. Security invariants

Define what must hold across the relevant states and actors. Examples of properties to examine include value conservation, entitlement consumed at most once, agreement between global and individual accounting, and consistent application of authorization policy.

An invariant guides the review; it is not an executed check until it is bound to concrete code, setup and assertions. Prioritize hypotheses by potential impact, reachability and uncertainty, rather than promoting every suspicious pattern.

## 3. Targeted tests

Choose the smallest experiment that addresses the hypothesis. Reuse protocol fixtures when appropriate, make actors and dependencies explicit, and retain mocks and privileged setup as limitations. A local fork should identify its block and the actions actually executed.

Inspect assertions, before-and-after measurements, traces and state changes. Determine whether the test reached its body and whether PASS means a safety property, an observed anomaly or an expected failure. [Testing approach](../research-tools-testing/testing.md).

## 4. Counter-tests

Test the strongest alternative explanation. Compare the supported flow, expected ordering, remaining permission checks and relevant prior findings. Separate design choices and trusted-role behavior from an attacker-controlled path.

Record rejected ideas and the condition that would justify reopening them. A negative result is useful when its tested scope and assumptions are clear. [Lifecycle overview](../protocol-investigations/reward-lifecycle.md) · [Temporal-state overview](../protocol-investigations/temporal-state.md).

## 5. Economic impact qualification

Identify who can act, who loses, who gains and where the value originates. Distinguish a local primitive from a complete exploitation path. State unpriced costs, synthetic state, missing integration steps and uncertain production configuration.

A demonstrated behavior can remain an observation when its impact is not established. Test success and external acceptance are separate questions. [Economic controls](../protocol-investigations/economic-counter-tests.md) · [Finding validation](validation.md).

## Audit-OS and research assistance

Audit-OS is my working environment for scope notes, architecture maps, invariants, hypotheses, tests, traces and conclusions. Target-specific scripts help collect data, compare sources and prepare evidence; this is not a claim of a unified autonomous audit engine.

I use **Codex as research assistance** for code navigation, analysis and drafting tests. Suggestions and generated code require source review and technical verification. An AI explanation is not a test result, a generated PoC is not a successful execution, and neither establishes independent validation.

## Executable example

The [original Refund Escrow lab](../educational-demos/refund-escrow/README.md) maps this workflow to code: a backing invariant, callback hypothesis, targeted experiment, control cases, economic attribution and correction. Its [actual execution record](../educational-demos/refund-escrow/EVIDENCE.md) includes a passing default suite, a deliberately failing vulnerable safety assertion and a bounded fixed-version invariant campaign. The [research-process page](../educational-demos/refund-escrow/RESEARCH_PROCESS.md) states Codex's contribution and the verification boundary. This is a teaching artifact, not an audit finding on a third-party protocol.

## Public evidence

The [Injective result](../validated-findings/injective-peggy-bridge/README.md) is the externally verified outcome presented here. The investigation overviews explain methods without disclosing the underlying private technical evidence; they do not imply those techniques produced the Injective finding. Detailed third-party studies and runnable protocol exploits remain withheld pending disclosure review; the independently created teaching example is available above.

[Research workflow](workflow.md) · [Finding validation](validation.md) · [Research tools](../research-tools-testing/README.md) · [Portfolio](../README.md)
