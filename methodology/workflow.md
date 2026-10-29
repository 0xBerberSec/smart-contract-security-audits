# Research Workflow

| Stage | Work product | Decision |
| --- | --- | --- |
| Scope and provenance | Source revision, contracts, dependencies and exclusions | Is the relevant implementation available and attributable? |
| Architecture and value | Actor map, call paths and accounting model | Which paths move assets or change entitlements? |
| Invariants and hypotheses | Explicit property, state and suspected failure | What concrete observation would support or refute the idea? |
| Experiment | Fixture, actors, assertions, command and result | Did execution reach the property being tested? |
| Trace and accounting review | Before/after deltas, events and relevant state | What changed, and why? |
| Counter-test | Alternative order, baseline or competing explanation | Does the original interpretation survive? |
| Qualification | Preconditions, bounded impact, source limitations and prior-art comparison | Finding, observation, hypothesis, rejected path or blocked research? |
| Reporting | Evidence, conclusion, uncertainty and reopening condition | What may be claimed and what may be disclosed? |

This describes a research practice, not a claim of exhaustive coverage or identical execution across every investigation. Historical evidence stays separate from fresh reproduction. A missing RPC or dependency remains a verification limit rather than a successful check.

[Methodology](README.md) · [Testing](../research-tools-testing/testing.md) · [Validation](validation.md) · [Portfolio](../README.md)
